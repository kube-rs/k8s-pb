VERSION := "1.28.4"

default:
  @just --list

# Download protos schemas from upstream
protos-dl:
    #!/usr/bin/env bash
    set -exuo pipefail
    cd k8s-pb-codegen
    rm -rf protos && mkdir protos && cd protos
    for x in api apimachinery apiextensions-apiserver kube-aggregator metrics; do
        mkdir -p ./$x
        curl -sSL https://github.com/kubernetes/$x/archive/refs/tags/kubernetes-{{VERSION}}.tar.gz | tar xzf - -C ./$x/ --strip-components=1
        fd -e proto -x sh -c "mkdir -p k8s.io/'{//}'; mv '{}' k8s.io/'{}'" ';' . ./$x
        rm -rf ./$x
    done

# Patch protos schemas to fix import paths
protos-patch:
    #!/usr/bin/env bash
    set -exuo pipefail
    cd k8s-pb-codegen
    fd -e proto -x sd 'k8s\.io\.(.+);' '$1;' {}
    fd -e proto -x sd 'import "k8s\.io/(.+)";' 'import "$1";' {}
    mv protos/k8s.io/* protos/
    rmdir protos/k8s.io/

# Generate protos path list for prost
protos-list:
    #!/usr/bin/env bash
    set -exuo pipefail
    cd k8s-pb-codegen
    fd -e proto | sort > protos.list

# Download and generate all protos dependent files
protos: protos-dl protos-patch protos-list

# Download swagger
swagger-dl:
    #!/usr/bin/env bash
    set -exuo pipefail
    curl -sSL -o k8s-pb-codegen/openapi/swagger.json \
        https://raw.githubusercontent.com/kubernetes/kubernetes/v{{VERSION}}/api/openapi-spec/swagger.json

# Patch swagger schema for upstream bugs
swagger-patch:
    #!/usr/bin/env bash
    set -exuo pipefail
    cd k8s-pb-codegen/openapi
    jq -f patches/patch-nonexistent-gvk.jq < swagger.json > swagger-patched.json
    mv swagger-patched.json swagger.json

# Transform swagger schema into api-resources.json
swagger-transform:
    #!/usr/bin/env bash
    set -exuo pipefail
    cd k8s-pb-codegen/openapi
    jq -f transform.jq < swagger.json > transformed.json

# Download and generate all swagger dependent files
swagger: swagger-dl swagger-patch swagger-transform

# Generate the library code from completed swagger and protos
codegen:
    #!/usr/bin/env bash
    set -exuo pipefail
    rm -rf k8s-pb/src && mkdir k8s-pb/src
    # src/lib.rs must exist to `cargo run`
    touch k8s-pb/src/lib.rs
    cd k8s-pb-codegen
    rm -rf tmp/ && mkdir tmp
    cargo run
