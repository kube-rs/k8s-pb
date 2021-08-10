[
  .paths | to_entries[]
  | .key as $path
  | .value | to_entries[]
  # Only process path infos with GVK (methods) and ignore deprecated.
  | .value["x-kubernetes-group-version-kind"]? as $gvk
  | select($gvk != null and (.value.description | test("deprecated: "; "i") | not))
  # Fall back to method name.
  | (.value["x-kubernetes-action"] // .key) as $verb
  | {
    path: $path,
    verb: (if $verb == "post" then "create" elif $verb == "put" then "update" else $verb end),
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
      | sub("^/apis?/(?:\($gvk.group)/)?\($gvk.version)/(?:namespaces/\\{namespace\\}/)?"; "")
      | split("/")
      | map(select(. | (startswith("{") | not)))
      | join("/")
    ),
  }
]
