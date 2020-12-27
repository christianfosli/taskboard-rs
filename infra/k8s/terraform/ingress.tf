resource "kubernetes_namespace" "ingressNamespace" {
  metadata {
    name = "ingress-nginx"
  }
}

resource "helm_release" "ingressNginx" {
  name        = "ingress-nginx"
  namespace   = kubernetes_namespace.ingressNamespace.metadata.0.name
  repository  = "https://kubernetes.github.io/ingress-nginx"
  chart       = "ingress-nginx"
  version     = "~>3.17"
  max_history = 5

  set {
    name  = "controller.metrics.enabled"
    value = true
  }
}

resource "kubectl_manifest" "ingress" {
  yaml_body = file("ingress.yaml")
}
