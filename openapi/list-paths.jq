[
  .paths | to_entries[] |
  .key as $path |
  .value | to_entries[] |
  # Only process path infos with GVK (methods) and ignore deprecated.
  .value["x-kubernetes-group-version-kind"]? as $gvk |
  select($gvk != null and (.value.description | test("deprecated: "; "i") | not)) |
  {
    path: $path,
    # Fall back to method name.
    verb: (.value["x-kubernetes-action"] // .key),
    group: $gvk.group,
    version: $gvk.version,
    kind: $gvk.kind,
    namespaced: ($path | test("/namespaces/\\{namespace\\}/")),
    subresource: ($path | test("\\{name\\}/")),
    consumes: (.value.consumes? // ["*/*"] | join(", ")),
    produces: (.value.produces? // ["*/*"] | join(", ")),
    # Path segments for plural names. Includes subresource as well.
    segments: (
      $path |
      sub("^/apis?/(?:\($gvk.group)/)?\($gvk.version)/(?:namespaces/\\{namespace\\}/)?"; "") |
      split("/") |
      map(select(. | (startswith("{") | not)))
    ),
  }
]
