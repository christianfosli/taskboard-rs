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
