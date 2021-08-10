[
  .paths | to_entries[]
  | .key as $path
  | .value | to_entries[]
  # Only process path infos with GVK (methods) and ignore deprecated.
  | .value["x-kubernetes-group-version-kind"]? as $gvk
  | select($gvk != null and (.value.description | test("deprecated: "; "i") | not))
  # Fall back to method name.
  | .key as $method
  | (.value["x-kubernetes-action"] // $method) as $verb
  | {
    # Plural name. Includes a subresource name like in `APIResourceList`.
    name: (
      $path
      | sub("^/apis?/(?:\($gvk.group)/)?\($gvk.version)/(?:namespaces/\\{namespace\\}/)?"; "")
      | split("/")
      | map(select(. | (startswith("{") | not)))
      | join("/")
    ),
    namespaced: ($path | test("/namespaces/\\{namespace\\}/")),
    kind: $gvk.kind,
    verb: (if $verb == "post" then "create" elif $verb == "put" then "update" else $verb end),
    # TODO Subresource's group verion can be different. e.g., `apps/v1` can contain subresource `autoscaling/v1` `Scale`.
    #      `APIResourceList` adds `group` and `version` field in that case.
    #      Use `path` to find API group version, then use GVK for each resource.
    groupVersion: ([$gvk.group, $gvk.version] | map(select(. != "")) | join("/")),
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
