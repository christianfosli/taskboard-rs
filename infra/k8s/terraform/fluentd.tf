resource "kubectl_manifest" "fluentd" {
  yaml_body = file("fluentd.yaml")
}

resource "random_password" "fluent" {
  length           = 16
  special          = true
  override_special = "()-_%"
}

resource "kubernetes_secret" "taskboardEsFluentUser" {
  metadata {
    name      = "taskboard-es-fluent-user"
    namespace = "kube-system"
    labels = {
      "k8s-app" = "fluentd-logging"
    }
  }

  data = {
    username = "fluent"
    password = random_password.fluent.result
  }
}

# MANUAL STEP: Create a user in elastic (i.e. through UI in Kibana)
# that matches the credentials in the secret above
