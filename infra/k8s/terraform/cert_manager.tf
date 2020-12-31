resource "kubernetes_namespace" "certManagerNamespace" {
  metadata {
    name = "cert-manager"
  }
}

resource "helm_release" "certManager" {
  name        = "cert-manager"
  namespace   = kubernetes_namespace.certManagerNamespace.metadata.0.name
  repository  = "https://charts.jetstack.io"
  chart       = "cert-manager"
  version     = "~>1.1"
  max_history = 5

  set {
    name  = "installCRDs"
    value = true
  }
}

resource "kubectl_manifest" "tlsCertIssuer" {
  yaml_body  = file("acme-issuer.yaml")
  depends_on = [helm_release.certManager]
}

resource "kubectl_manifest" "tlsCert" {
  yaml_body  = file("tls-cert.yaml")
  depends_on = [helm_release.certManager]
}
