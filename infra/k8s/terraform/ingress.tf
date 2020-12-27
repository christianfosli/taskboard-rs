resource "helm_release" "ingressNginx" {
  name       = "ingress-nginx"
  namespace  = "ingress-nginx"
  repository = "https://kubernetes.github.io/ingress-nginx"
  chart      = "ingress-nginx"
  version    = "~>3.17"

  set {
    name  = "controller.metrics.enabled"
    value = true
  }
}

resource "kubectl_manifest" "ingress" {
  yaml_body = file("ingress.yml")
}
