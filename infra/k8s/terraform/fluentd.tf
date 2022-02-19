resource "kubernetes_manifest" "fluentd" {
  manifest = yamldecode(file("fluentd.yaml"))
}
