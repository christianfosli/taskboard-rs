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

resource "kubectl_manifest" "elasticSearchCluster" {
  yaml_body  = file("elasticsearch-cluster.yaml")
  depends_on = [helm_release.eckOperator]
}
