resource "helm_release" "certManager" {
  name       = "cert-manager"
  namespace  = "cert-manager"
  repository = "https://charts.jetstack.io"
  chart      = "cert-manager"
  version    = "~>1.1"

  set {
    name  = "installCRDs"
    value = true
  }
}

resource "kubectl_manifest" "tlsCertIssuer" {
  yaml_body  = <<YAML
apiVersion: cert-manager.io/v1
kind: ClusterIssuer
metadata:
  name: letsencrypt
spec:
  acme:
    email: cfosli@gmail.com
    server: https://acme-v02.api.letsencrypt.org/directory
    privateKeySecretRef:
      name: letsencrypt
    solvers:
    - http01:
        ingress:
          class: nginx
YAML
  depends_on = [helm_release.certManager]
}
