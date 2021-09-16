# Fix path operation annotated with a `x-kubernetes-group-version-kind` that references a type that doesn't exist in the schema.
# See https://github.com/Arnavion/k8s-openapi/blob/445e89ec444ebb1c68e61361e64eec4c4a3f4785/k8s-openapi-codegen/src/fixups/upstream_bugs.rs#L9
(.paths | .. | objects | select((.group? == "") and (.version? == "v1") and (.kind? | type) == "string")).kind |= (
  if . | test("^Pod(?:Attach|Exec|PortForward|Proxy)Options$") then
    "Pod"
  elif . == "NodeProxyOptions" then
    "Node"
  elif . == "ServiceProxyOptions" then
    "Service"
  else
    .
  end
)
