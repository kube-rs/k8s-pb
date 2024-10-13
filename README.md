# k8s-pb
[![Crates.io](https://img.shields.io/crates/v/k8s-pb.svg)](https://crates.io/crates/k8s-pb)

Kubernetes protobuf bindings for [kube-rs](https://github.com/kube-rs).
WIP. Not yet useable as is.

## Usage
This library is **not currently usable** from `kube`.
For now consider these structs a reference location for Kubernetes structs that are not found in `k8s-openapi`.
See [docs.rs/k8s-pb](https://docs.rs/k8s-pb/latest/k8s_pb/).

## Build Process
The code generation process consists of 4 steps;

1. `just protos` - download and patch protobufs
2. `just swagger` - download and transform openapi schema
3. `just codegen` - combine info and build with prost
4. `just names` - case alignment and `cargo fmt`

This can be run in one step with `just generate`.

The [k8s-pb](https://github.com/kube-rs/k8s-pb/tree/main/k8s-pb) crate is generated as a result of this process and then published periodically.

### just protos
Obtains the [version pinned](https://github.com/kube-rs/k8s-pb/blob/main/justfile#L1) protobufs from upstream:

- https://github.com/kubernetes/api/releases
- https://github.com/kubernetes/apimachinery/releases
- https://github.com/kubernetes/apiextensions-apiserver/releases
- https://github.com/kubernetes/kube-aggregator/releases
- https://github.com/kubernetes/metrics/releases

then does [minor transforms](https://github.com/kube-rs/k8s-pb/blob/main/justfile#L19-L34) to prepare them for building.

### just swagger
Obtains the [version pinned](https://github.com/kube-rs/k8s-pb/blob/main/justfile#L1) swagger openapi schema from upstream:

- https://github.com/kubernetes/kubernetes/tree/master/api/openapi-spec

then applies any needed [patches](https://github.com/kube-rs/k8s-pb/tree/main/k8s-pb-codegen/openapi/patches) before [transforming](https://github.com/kube-rs/k8s-pb/blob/main/k8s-pb-codegen/openapi/transform.jq) the schema into a shorter [json file](https://github.com/kube-rs/k8s-pb/blob/main/k8s-pb-codegen/openapi/transformed.json) containing desired generic information.

This json file complements the protos with type type properties needed for trait implementations.

### just codegen
Runs [main.rs](https://github.com/kube-rs/k8s-pb/blob/main/k8s-pb-codegen/src/main.rs), using the outputs from the `swagger` and `protobuf` recipes above. In particular;

- The protos are built with [prost](https://github.com/tokio-rs/prost) via `protoc` and provides a [`FileDescriptorSet`](https://docs.rs/prost-types/latest/prost_types/struct.FileDescriptorSet.html) via [`Config::file_descriptor_set_path`](https://docs.rs/prost-build/latest/prost_build/struct.Config.html#method.file_descriptor_set_path).
- The transformed swagger result json is deserialized through [lib.rs](https://github.com/kube-rs/k8s-pb/blob/main/k8s-pb-codegen/src/lib.rs) into a `HashMap<String, Resource>` where the string is a GVK string.
- inject generics for each modules in hashmap via `append_trait_impl`
- Generate a module tree from the seen modules in the loop
- Attach our implemented traits to the [generated lib.rs](https://github.com/kube-rs/k8s-pb/blob/main/k8s-pb/src/lib.rs)

### Build Dependencies

- [fd](https://github.com/sharkdp/fd)
- [jq](https://stedolan.github.io/jq/)
- [just](https://github.com/casey/just)
- [sd](https://github.com/chmln/sd)
- [protoc](https://github.com/protocolbuffers/protobuf)
- [rustmft](https://github.com/rust-lang/rustfmt)
