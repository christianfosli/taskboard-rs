resource "kubernetes_namespace" "monitoring" {
  metadata {
    name = "monitoring"
  }
}

resource "helm_release" "prometheus" {
  name        = "prometheus"
  namespace   = kubernetes_namespace.monitoring.metadata.0.name
  repository  = "https://prometheus-community.github.io/helm-charts"
  chart       = "prometheus-stack"
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
  yaml_body = <<YAML
apiVersion: monitoring.coreos.com/v1
kind: ServiceMonitor
metadata:
  name: prom-monitor-ingress
  namespace: ${kubernetes_namespace.monitoring.metadata.0.name}
  labels:
    app: ingress-nginx
    # kube-prometheus-stack helm operator configures prometheus to only look
    # for ServiceMonitor/PodMonitor tagged with their release name
    release: prometheus-stack
spec:
  selector:
    matchLabels:
      app.kubernetes.io/name: ingress-nginx
  endpoints:
  - port: metrics
YAML
}
