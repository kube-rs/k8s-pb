[
  .paths | to_entries[]
  | .key as $path
  | .value | to_entries[]
  # Only process path infos with GVK (methods) and ignore deprecated.
  | .value["x-kubernetes-group-version-kind"]? as $gvk
  | select($gvk != null and (.value.description | test("deprecated: "; "i") | not))
  # Keep group and version from path because subresource's GVK might be different. e.g., `autoscale/v1` subresource in `apps/v1`.
  | ($path | capture("^/(?:(?:api/(?<coreVersion>[^/]+))|(?:apis/(?<group>[^/]+)/(?<version>[^/]+)))/")) as $gv
  | (if $gv.coreVersion != null then "\($gv.coreVersion)" else "\($gv.group)/\($gv.version)" end) as $groupVersion
  # Fall back to method name.
  | (.value["x-kubernetes-action"] // .key) as $verb
  | {
    path: $path,
    verb: (if $verb == "post" then "create" elif $verb == "put" then "update" else $verb end),
    groupVersion: $groupVersion,
    group: $gvk.group,
    version: $gvk.version,
    kind: $gvk.kind,
    namespaced: ($path | test("/namespaces/\\{namespace\\}/")),
    subresource: ($path | test("\\{name\\}/")),
    consumes: (.value.consumes? // ["*/*"] | join(", ")),
    produces: (.value.produces? // ["*/*"] | join(", ")),
    # Plural name. Includes a subresource name like in `APIResourceList`.
    name: (
      $path
      | sub("^/apis?/\($groupVersion)/(?:namespaces/\\{namespace\\}/)?"; "")
      | split("/")
      | map(select(. | (startswith("{") | not)))
      | join("/")
    ),
  }
]
