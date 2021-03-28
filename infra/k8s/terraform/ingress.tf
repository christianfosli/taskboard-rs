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
  version     = "~>3.25"
  max_history = 5

  set {
    name  = "controller.podAnnotations"
    value = "linkerd.io/inject: enabled"
  }
}

resource "kubectl_manifest" "appIngress" {
  yaml_body = file("app-ingress.yaml")
}

resource "kubectl_manifest" "apiIngress" {
  yaml_body = file("api-ingress.yaml")
}
