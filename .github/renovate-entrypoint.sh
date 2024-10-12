#!/bin/bash
set -exuo pipefail

# renovate: datasource=github-releases depName=clux/whyq
export YQ_VER=0.10.2
# renovate: datasource=github-releases depName=casey/just
export JUST_VER=1.28.0
# renovate: datasource=github-releases depName=BurntSushi/ripgrep
export RG_VER=14.1.0
# renovate: datasource=github-releases depName=sharkdp/fd
export FD_VER=10.2.0
# renovate: datasource=github-releases depName=chmln/sd
export SD_VER=1.0.0

apt update
apt install -y curl git protobuf-compiler

curl -sSL https://github.com/clux/whyq/releases/download/${YQ_VER}/yq-x86_64-unknown-linux-musl.tar.xz | tar xJ --strip-components=1 -C /usr/local/bin
curl -sSL https://github.com/casey/just/releases/download/${JUST_VER}/just-${JUST_VER}-x86_64-unknown-linux-musl.tar.gz | tar xz -C /usr/local/bin
curl -sSL https://github.com/BurntSushi/ripgrep/releases/download/${RG_VER}/ripgrep-${RG_VER}-x86_64-unknown-linux-musl.tar.gz | tar xz --strip-components=1 -C /usr/local/bin
curl -sSL https://github.com/sharkdp/fd/releases/download/v${FD_VER}/fd-v${FD_VER}-x86_64-unknown-linux-musl.tar.gz | tar xz --strip-components=1 -C /usr/local/bin/ --wildcards '*fd'
curl -sSL https://github.com/chmln/sd/releases/download/v${SD_VER}/sd-v${SD_VER}-x86_64-unknown-linux-musl.tar.gz | tar xz --strip-components=1 -C /usr/local/bin/ --wildcards '*sd'

runuser -u ubuntu renovate
