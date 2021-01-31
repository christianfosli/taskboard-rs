resource "kubernetes_namespace" "elasticNamespace" {
  metadata {
    name = "elastic-system"
  }
}

resource "helm_release" "eckOperator" {
  name         = "eck-operator"
  namespace    = kubernetes_namespace.elasticNamespace.metadata.0.name
  repository   = "https://helm.elastic.co"
  chart        = "eck-operator"
  version      = "~>1.3"
  force_update = true
  max_history  = 5
}

# --- The below resources should ideally be put into a separate namespace
#     just putting them in default for now, since that makes it easy for
#     microservices to get the credentials for the elasticsearch cluster ---

resource "kubectl_manifest" "elasticSearchCluster" {
  yaml_body  = file("elasticsearch.yaml")
  depends_on = [helm_release.eckOperator]
}

# Manual Step when running on a single node:
# From within K8S cluster do
# PUT <elasticsearch-url>/_template/number_of_replicas
# {
#   "template": "*",
#   "settings": {
#     "number_of_replicas": 0
#   }
# }
#
# This prevents updates from getting stuck because no nodes can be removed
#
# GET <elasticsearch-url>/_cat/indices  should all be green!
