Experimenting with Kubernetes protobufs.

## Download

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

Collect all paths to generate:

```bash
# In project root.
fd -e proto -x echo '"{}",' | sort
# Copy the output to `build.rs`
```
