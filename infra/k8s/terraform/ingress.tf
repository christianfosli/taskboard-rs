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
  version     = "4"
  max_history = 5

  set {
    name  = "controller.podAnnotations.linkerd\\.io/inject"
    value = "enabled"
  }

  set {
    name  = "controller.podAnnotations.config\\.linkerd\\.io/proxy-cpu-request"
    value = "10m"
  }

  set {
    name  = "controller.podAnnotations.config\\.linkerd\\.io/proxy-memory-request"
    value = "5Mi"
  }

  set {
    name  = "controller.podAnnotations.kubectl\\.kubernetes\\.io/default-logs-container"
    value = "controller"
  }
}

resource "kubectl_manifest" "appIngress" {
  yaml_body = file("app-ingress.yaml")
}

resource "kubectl_manifest" "apiIngress" {
  yaml_body = file("api-ingress.yaml")
}
