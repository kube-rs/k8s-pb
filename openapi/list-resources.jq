[
  .paths | to_entries[]
  | .key as $path
  | .value | to_entries[]
  # Only process path infos with GVK (methods) and ignore deprecated.
  | .value["x-kubernetes-group-version-kind"]? as $gvk
  | select($gvk != null and (.value.description | test("deprecated: "; "i") | not))
  # Use group and version from path to group by because subresource's GVK might be different. e.g., `autoscale/v1` in `apps/v1`.
  | ($path | capture("^/(?:(?:api/(?<coreVersion>[^/]+))|(?:apis/(?<group>[^/]+)/(?<version>[^/]+)))/")) as $gv
  | (if $gv.coreVersion != null then "\($gv.coreVersion)" else "\($gv.group)/\($gv.version)" end) as $groupVersion
  # Fall back to method name.
  | .key as $method
  | (.value["x-kubernetes-action"] // $method) as $verb
  | {
    # Plural name. Includes a subresource name like in `APIResourceList`.
    name: (
      $path
      | sub("^/apis?/\($groupVersion)/(?:namespaces/\\{namespace\\}/)?"; "")
      | split("/")
      | map(select(. | (startswith("{") | not)))
      | join("/")
    ),
    namespaced: ($path | test("/namespaces/\\{namespace\\}/")),
    kind: $gvk.kind,
    verb: (if $verb == "post" then "create" elif $verb == "put" then "update" else $verb end),
    groupVersion: $groupVersion,
    group: $gvk.group,
    version: $gvk.version,
  }
]
| group_by(.groupVersion)
| map({
  groupVersion: .[0].groupVersion,
  resources: (
    group_by(.name)
    | map({
      name: .[0].name,
      namespaced: .[0].namespaced,
      group: .[0].group,
      version: .[0].version,
      kind: .[0].kind,
      verbs: ([.[] | .verb] | unique),
    })
  )
})
