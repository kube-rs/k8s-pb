# Transform `swagger.json` to a map of proto path to resource infos.
#
# Uses `x-kubernetes-group-version-kind` to join `.definitions` and `.paths`.
#
# We should be able to find the following:
#
# - GVK
# - Plural name (path segments)
#  - `singularName` in `APIResourceList` seems to be always empty for builtins. It's used by `kubectl` for CRDs? ([kubernetes/kubernetes#18622](https://github.com/kubernetes/kubernetes/issues/18622#issuecomment-434481731))
# - Supported verbs
#   - `x-kubernetes-action` can be one of `get`, `list`, `put`, `patch`, `post`, `delete`, `deletecollection`, `watch`, `watchlist`, `proxy`, or `connect`. `verbs` in `APIResourceList` uses `create` instead of `post` and `update` instead of `put`? No `connect`?
# - Supported content types
# - Scope
#   - Namespaced if any possible path contains `/namespaces/{namespace}/`
#     - May also have paths for all namespaces for some verbs (e.g., `list` all pods)
#   - Subresource if path contains `/{name}/` (`/` after `{name}`)
#
# See https://github.com/kubernetes/kubernetes/tree/master/api/openapi-spec for more information.

def fmap(f): if . != null then . | f else . end;
def to_rust: . | sub("^io\\.k8s\\."; "") | gsub("-"; "_") | gsub("\\."; "::");
def strip_ref_prefix: . | sub("^#/definitions/"; "");
# GVK object to slash separated string.
def gvk_string: [.group, .version, .kind] | map(select(. != "")) | join("/");

(
  [
    .definitions as $defs |
    .definitions |
    to_entries[] |
    # Only process definitions with GVK array.
    # Exclude List. .properties.metadata.$ref "#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.ListMeta"
    .value["x-kubernetes-group-version-kind"]? as $gvks |
    select(
      $gvks != null and
      ($gvks | length == 1) and
      (.value.properties?.metadata?["$ref"]? != "#/definitions/io.k8s.apimachinery.pkg.apis.meta.v1.ListMeta")
    ) |
    (.value.properties?.metadata?["$ref"] | fmap(strip_ref_prefix | to_rust)) as $metadata |
    (.value.properties?.spec?["$ref"] | fmap(strip_ref_prefix | to_rust)) as $spec |
    (.value.properties?.status?["$ref"] | fmap(strip_ref_prefix)) as $statusName |
    (
      $statusName |
      fmap($defs[.].properties?.conditions?.items?["$ref"]) |
      fmap(strip_ref_prefix | to_rust)
    ) as $condition |
    {
      key: $gvks[0] | gvk_string,
      value: {
        proto: .key | sub("^io\\.k8s\\."; "") | gsub("-"; "_"),
        rust: .key | to_rust,
        metadata: $metadata,
        spec: $spec,
        status: $statusName | fmap(to_rust),
        condition: $condition,
      },
    }
  ] |
  sort_by(.key) |
  from_entries
) as $definitions |

[
  .paths |
  to_entries[] |
  .key as $path |
  .value |
  to_entries[] |
  # Only process path infos with GVK (methods) and ignore deprecated `/watch/` paths that doesn't fit the pattern.
  .value["x-kubernetes-group-version-kind"]? as $gvk |
  select($gvk != null and (.value.description | test("deprecated: use the 'watch'"; "i") | not)) |
  # Use group and version from path to group by because subresource's GVK might be different.
  # e.g., `autoscale/v1` in `apps/v1`.
  (
    $path |
    capture("^/(?:(?:api/(?<coreVersion>[^/]+))|(?:apis/(?<group>[^/]+)/(?<version>[^/]+)))/")
  ) as $gv |
  (
    if $gv.coreVersion != null then
      "\($gv.coreVersion)"
    else
      "\($gv.group)/\($gv.version)"
    end
  ) as $apiGroupVersion |
  # Fall back to method name.
  (.value["x-kubernetes-action"] // .key) as $verb |
  $definitions[$gvk | gvk_string] as $definition |
  {
    # Plural name. Includes a subresource name like in `APIResourceList`.
    name: (
      $path |
      sub("^/apis?/\($apiGroupVersion)/(?:namespaces/\\{namespace\\}/)?"; "") |
      split("/") |
      map(select(. | (startswith("{") | not))) |
      join("/")
    ),
    namespaced: ($path | test("/namespaces/\\{namespace\\}/")),
    kind: $gvk.kind,
    verb: (if $verb == "post" then "create" elif $verb == "put" then "update" else $verb end),
    apiGroupVersion: $apiGroupVersion,
    group: $gvk.group,
    version: $gvk.version,
    subresource: ($path | test("\\{name\\}/")),
    proto: $definition.proto,
    rust: $definition.rust,
    metadata: $definition.metadata,
    spec: $definition.spec,
    status: $definition.status,
    condition: $definition.condition,
    path: $path,
  }
] |
# Group resources by `apiGroupVersion` like `APIResourceList`, then
# combine subresources within the group with the parent.
group_by(.apiGroupVersion) |
map({
  apiGroupVersion: .[0].apiGroupVersion,
  # Collect all `paths` and `scopedVerbs` for this resource/subresource.
  resources: (
    group_by(.name) |
    map({
      name: .[0].name,
      # Some resources can be both namespaced and cluster scoped.
      # `namespaced` is true if it can be namespaced.
      namespaced: (map(.namespaced) | any),
      subresource: .[0].subresource,
      apiVersion: .[0].apiGroupVersion,
      group: .[0].group,
      version: .[0].version,
      kind: .[0].kind,
      proto: .[0].proto,
      rust: .[0].rust,
      metadata: .[0].metadata,
      spec: .[0].spec,
      status: .[0].status,
      condition: .[0].condition,
      scopedVerbs: (
        group_by(.namespaced) |
        map({
          key: (if .[0].namespaced then "namespaced" else "all" end),
          value: (map(.verb) | unique)
        }) |
        from_entries
      ),
      paths: (map(.path) | unique | sort_by(length)),
    }) |
    # Add subresource infos to parent and remove them
    ([.[] | select(.subresource) | {name, scopedVerbs, paths}]) as $subresources |
    [
      .[] |
      select(.subresource | not) |
      (.name + "/") as $parent |
      . +
      {
        subresources: [
          $subresources |
          .[] |
          select(.name | startswith($parent)) |
          {
            name: (.name | sub($parent; "")),
            scopedVerbs,
            paths
          }
        ]
      } |
      del(.subresource)
    ] as $resources |
    # basic sanity check
    if ($resources | map(.subresources | length) | add) == ($subresources | length) then
      $resources
    else
      error("some subresources were not associated with their parent")
    end
  )
}) |
[.[].resources[] | {key: .proto, value: .}] |
sort_by(.key) |
from_entries
