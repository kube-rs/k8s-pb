Experimenting with Kubernetes protobufs.

## Protobufs

### Download

Get protos by extracting them from Kubernetes releases:

- https://github.com/kubernetes/api/releases
- https://github.com/kubernetes/apimachinery/releases
- https://github.com/kubernetes/apiextensions-apiserver/releases
- https://github.com/kubernetes/kube-aggregator/releases
- https://github.com/kubernetes/metrics/releases

```bash
# In `protos/`
VERSION=1.22.0
for x in api apimachinery apiextensions-apiserver kube-aggregator metrics; do
  mkdir ./$x;
  curl -sSL https://github.com/kubernetes/$x/archive/refs/tags/kubernetes-$VERSION.tar.gz | tar xzf - -C ./$x/ --strip-components=1;
  fd -e proto -x sh -c "mkdir -p k8s.io/'{//}'; mv '{}' k8s.io/'{}'" ';' . ./$x;
  rm -rf ./$x;
done
```

### Patch

Removing `k8s.io.`:

```bash
fd -e proto -x sd 'k8s\.io\.(.+);' '$1;' {}
fd -e proto -x sd 'import "k8s\.io/(.+)";' 'import "$1";' {}
mv protos/k8s.io/* protos/
rmdir protos/k8s.io/
```

### Generate

Collect all paths to generate:

```bash
# In project root.
fd -e proto -x echo '"{}",' | sort
```
Copy the output to `build.rs`, then:

```bash
cargo build
```

### Hack

Generate a [`FileDescriptorSet`] containing all of the input files:

```bash
protoc \
    --include_imports \
    --include_source_info \
    --descriptor_set_out=k8s.pb \
    --proto_path=./protos \
    ./protos/**/*.proto
```

Working with `FileDescriptorSet`:
```rust
use prost_types::{FileDescriptorProto, FileDescriptorSet};
let buf = fs::read(fds_path).unwrap();
let fds = FileDescriptorSet::decode(&*buf).unwrap();
let files = fds.files;
```

See [`prost_build`](https://github.com/tokio-rs/prost/blob/32bc87cd0b7301f6af1a338e9afd7717d0f42ca9/prost-build/src/lib.rs#L765-L825).

[`FileDescriptorSet`]: https://github.com/tokio-rs/prost/blob/32bc87cd0b7301f6af1a338e9afd7717d0f42ca9/prost-types/src/protobuf.rs#L1-L7


## OpenAPI

We need to use `swagger.json` to fill in some information.

### Generics

Used for request path and de/serializing JSON.

Use `x-kubernetes-group-version-kind` to join `.definitions` and `.paths`.

We should be able to find the following:

- GVK
- Plural name (path segments)
  - `singularName` in `APIResourceList` seems to be always empty for builtins. It's used by `kubectl` for CRDs? ([kubernetes/kubernetes#18622](https://github.com/kubernetes/kubernetes/issues/18622#issuecomment-434481731))
- Supported verbs
  - `x-kubernetes-action` can be one of `get`, `list`, `put`, `patch`, `post`, `delete`, `deletecollection`, `watch`, `watchlist`, `proxy`, or `connect`. `verbs` in `APIResourceList` uses `create` instead of `post` and `update` instead of `put`? No `connect`?
- Supported content types
- Scope
  - Namespaced if path contains `/namespaces/{namespace}/`
  - Subresource if path contains `/{name}/`

### Download

In `openapi/`

```bash
VERSION=1.22.0
curl -sSL -o swagger.json \
    https://raw.githubusercontent.com/kubernetes/kubernetes/v$VERSION/api/openapi-spec/swagger.json
```

### Bug Fix

Fix path operation annotated with a `x-kubernetes-group-version-kind` that references a type that doesn't exist in the schema. (See [`k8s-openapi`](https://github.com/Arnavion/k8s-openapi/blob/445e89ec444ebb1c68e61361e64eec4c4a3f4785/k8s-openapi-codegen/src/fixups/upstream_bugs.rs#L9)).

```bash
gron swagger.json \
| perl -pe 's/(?<=kind = ")(Pod|Node|Service)(?:Attach|Exec|PortForward|Proxy)Options(?=")/$1/' \
| gron -u \
> swagger-patched.json 
mv swagger-patched.json swagger.json
```

### Transforming

Transform `swagger.json` to something easier to explore.

#### Like APIResourceList

```bash
cat swagger.json | jq -f list-resources.jq | jq '.[0]'
```

```json
{
  "groupVersion": "admissionregistration.k8s.io/v1",
  "resources": [
    {
      "name": "mutatingwebhookconfigurations",
      "namespaced": false,
      "group": "admissionregistration.k8s.io",
      "version": "v1",
      "kind": "MutatingWebhookConfiguration",
      "verbs": [
        "create",
        "delete",
        "deletecollection",
        "get",
        "list",
        "patch",
        "update"
      ]
    },
    {
      "name": "validatingwebhookconfigurations",
      "namespaced": false,
      "group": "admissionregistration.k8s.io",
      "version": "v1",
      "kind": "ValidatingWebhookConfiguration",
      "verbs": [
        "create",
        "delete",
        "deletecollection",
        "get",
        "list",
        "patch",
        "update"
      ]
    }
  ]
}
```

#### Paths

```typescript
type ResourcePath = {
  // Request path.
  path: string;
  // `x-kubernetes-action` or `method`.
  verb: string;
  // Group and version of API. Can be different from `group` and `version` for subresources.
  apiGroupVersion: string;
  // GVK
  group: string;
  version: string;
  kind: string;
  // True if the path contains `/namespaces/{namespace}/`
  namespaced: boolean;
  // True if the path contains a segment after `{name}`.
  subresource: boolean;
  // MIME types of supported content types. Comma separated.
  consumes: string;
  // MIME types of supported responses. Comma separated.
  produces: string;
  // Plural name. Includes subresources like APIResourceList.
  name: string;
};
```

```bash
cat swagger.json | jq -f list-paths.jq
```

```bash
cat swagger.json \
| jq -f list-paths.jq \
| jq 'map(select(.kind == "Pod" and .verb == "get" and .subresource == false))'
```

```json
[
  {
    "path": "/api/v1/namespaces/{namespace}/pods/{name}",
    "verb": "get",
    "apiGroupVersion": "v1",
    "group": "",
    "version": "v1",
    "kind": "Pod",
    "namespaced": true,
    "subresource": false,
    "consumes": "*/*",
    "produces": "application/json, application/yaml, application/vnd.kubernetes.protobuf",
    "name": "pods"
  }
]
```

Group by `name`:

```bash
cat swagger.json \
| jq -f list-paths.jq \
| jq 'group_by(.name)'
```
