Experimenting with Kubernetes protobufs.

## Build Dependencies

- [gron](https://github.com/tomnomnom/gron)
- [just](https://github.com/casey/just)
- [sd](https://github.com/chmln/sd)
- [jq](https://stedolan.github.io/jq/)

## Protobufs
We get protos by extracting them from pinned Kubernetes releases:

- https://github.com/kubernetes/api/releases
- https://github.com/kubernetes/apimachinery/releases
- https://github.com/kubernetes/apiextensions-apiserver/releases
- https://github.com/kubernetes/kube-aggregator/releases
- https://github.com/kubernetes/metrics/releases

We then do minor transforms on top of that to prepare for building.
Results of this step is committed already. But to run, invoke `just protos`

## Openapi
To complement the protos with generic information, we also download the swagger schema, patch it, and transform it as described below.

Results of this step is committed already. But to run, invoke `just swagger`.


## Building
To build the [out](./out) directory from [build.rs](./build.rs) using swagger and protobuf results run `just build`.

Results of this step is committed already.

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


## OpenAPI Strategy

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
  - Namespaced if any possible path contains `/namespaces/{namespace}/`
    - May also have paths for all namespaces for some verbs (e.g., `list` all pods)
  - Subresource if path contains `/{name}/` (`/` after `{name}`)

### Transforming

Transform `swagger.json` to something easier to explore.

#### Like APIResourceList

```bash
cat swagger.json \
| jq -f list-resources.jq \
> api-resources.json
```

```bash
cat swagger.json | jq -f list-resources.jq | jq '.[0]'
```

```json
{
  "apiGroupVersion": "admissionregistration.k8s.io/v1",
  "resources": [
    {
      "name": "mutatingwebhookconfigurations",
      "namespaced": false,
      "apiGroupVersion": "admissionregistration.k8s.io/v1",
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
      "apiGroupVersion": "admissionregistration.k8s.io/v1",
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
