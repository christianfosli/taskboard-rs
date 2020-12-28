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
  yaml_body = <<YAML
apiVersion: monitoring.coreos.com/v1
kind: ServiceMonitor
metadata:
  name: prom-monitor-ingress
  namespace: ${kubernetes_namespace.monitoring.metadata.0.name}
  labels:
    app: ingress-nginx
    # kube-prometheus-stack helm chart configures prometheus such that
    # ServiceMonitor/PodMonitor must be tagged with their release name
    release: ${helm_release.prometheus.metadata.0.name}
spec:
  selector:
    matchLabels:
      app.kubernetes.io/name: ingress-nginx
  endpoints:
  - port: metrics
YAML
}

resource "kubectl_manifest" "grafanaIngress" {
  yaml_body = <<YAML
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: grafana-ingress
  namespace: ${kubernetes_namespace.monitoring.metadata.0.name}
  annotations:
    nginx.ingress.kubernetes.io/rewrite-target: /$2
spec:
  ingressClassName: nginx
  rules:
  - host: taskboard.cloud
    http:
      paths:
      - path: /grafana(/|$)(.*)
        pathType: Exact
        backend:
          service:
            name: prometheus-grafana
            port:
              number: 80
YAML
}
