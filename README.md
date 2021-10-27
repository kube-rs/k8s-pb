Experimenting with Kubernetes protobufs.

## Build Dependencies

- [fd](https://github.com/sharkdp/fd)
- [jq](https://stedolan.github.io/jq/)
- [just](https://github.com/casey/just)
- [sd](https://github.com/chmln/sd)

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
To build the [out](./out) directory from [build.rs](./build.rs) we will use the outputs from the `swagger`, `protobuf`, and `protobuf-fds` targets.

Results of this step is committed already. But to run, invoke `just codegen`

### Hack

Generate a [`FileDescriptorSet`] containing all of the input files wih `just codegen-fds`


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
