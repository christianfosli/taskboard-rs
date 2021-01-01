resource "kubernetes_namespace" "monitoring" {
  metadata {
    name = "monitoring"
  }
}

resource "helm_release" "prometheus" {
  name        = "prometheus"
  namespace   = kubernetes_namespace.monitoring.metadata.0.name
  repository  = "https://prometheus-community.github.io/helm-charts"
  chart       = "kube-prometheus-stack"
  version     = "~>12.8"
  max_history = 5

  values = [
    <<YAML
grafana:
  grafana.ini:
    server:
      root_url: https://taskboard.cloud/grafana
YAML
  ]
}

resource "kubectl_manifest" "ingressServiceMonitor" {
  yaml_body = templatefile("ingress-service-monitor.yaml", {
    namespace         = kubernetes_namespace.monitor.metadata.0.name
    ingress_namespace = kubernetes_namespace.ingressNamespace.metadata.0.name
    release_name      = helm_release.prometheus.metadata.0.name
    ingress_app_name  = helm_release.ingressNginx.chart
  })
}

resource "kubectl_manifest" "grafanaIngress" {
  yaml_body = templatefile("grafana-ingress.yaml", {
    namespace = kubernetes_namespace.monitoring.metadata.0.name
  })
}
