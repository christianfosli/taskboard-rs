resource "kubernetes_namespace" "elasticNamespace" {
  metadata {
    name = "elastic-system"
  }
}

resource "helm_release" "eckOperator" {
  name        = "eck-operator"
  namespace   = kubernetes_namespace.elasticNamespace.metadata.0.name
  repository  = "https://helm.elastic.co"
  chart       = "eck-operator"
  version     = "~>1.3.1"
  max_history = 5
}

# --- The below resources should ideally be put into a separate namespace
#     just putting them in default for now, since that makes it easy for
#     microservices to get the credentials for the elasticsearch cluster ---

resource "kubectl_manifest" "elasticSearchCluster" {
  yaml_body  = file("elasticsearch.yaml")
  depends_on = [helm_release.eckOperator]
}

resource "kubectl_manifest" "kibana" {
  yaml_body = templatefile("kibana.yaml", {
    elasticsearch_name = kubectl_manifest.elasticSearchCluster.name
  })
}

resource "kubectl_manifest" "elasticBeats" {
  yaml_body = templatefile("elastic-beats.yaml", {
    elasticsearch_name = kubectl_manifest.elasticSearchCluster.name
    kibana_name        = kubectl_manifest.kibana.name
  })
}

resource "kubectl_manifest" "kibanaIngress" {
  yaml_body = file("kibana-ingress.yaml")
}
