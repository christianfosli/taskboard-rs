variable "LINKERD_VERSION" {
  default     = "~>2.10.1"
  description = "The version of Linkerd to install"
}

resource "kubernetes_namespace" "linkerdNamespace" {
  metadata {
    name = "linkerd"
    labels = {
      "config.linkerd.io/admission-webhooks" = "disabled"
    }
  }
}

resource "helm_release" "linkerd" {
  name        = "linkerd"
  namespace   = kubernetes_namespace.linkerdNamespace.metadata.0.name
  repository  = "https://helm.linkerd.io/stable"
  chart       = "linkerd2"
  version     = var.LINKERD_VERSION
  max_history = 5

  set {
    name  = "identityTrustAnchorsPEM"
    value = tls_self_signed_cert.linkerdTrustAnchor.cert_pem
  }

  set {
    name  = "identity.issuer.scheme"
    value = "kubernetes.io/tls"
  }

  set {
    name  = "installNamespace"
    value = "false"
  }
}

# --- Linkerd Viz ---

resource "helm_release" "linkerdViz" {
  name        = "linkerd-viz"
  repository  = "https://helm.linkerd.io/stable"
  chart       = "linkerd-viz"
  version     = var.LINKERD_VERSION
  max_history = 5

  depends_on = [helm_release.linkerd]
}

data "kubernetes_namespace" "linkerdVizNamespace" {
  metadata {
    name = "linkerd-viz"
  }
  depends_on = [helm_release.linkerdViz]
}

# --- Expose dashboard through ingress

resource "random_password" "linkerdDashboardPass" {
  length           = 32
  special          = true
  override_special = "_%@"
}

resource "kubernetes_secret" "linkerdDashboardBasicAuth" {
  metadata {
    name      = "web-ingress-auth"
    namespace = data.kubernetes_namespace.linkerdVizNamespace.metadata.0.name
  }
  data = {
    auth = "admin:${bcrypt(random_password.linkerdDashboardPass.result)}"
  }
}

resource "kubectl_manifest" "linkerdDashboardIngress" {
  yaml_body = templatefile("metrics-ingress.yaml", {
    secret          = kubernetes_secret.linkerdDashboardBasicAuth.metadata.0.name
    namespace       = data.kubernetes_namespace.linkerdVizNamespace.metadata.0.name
    servicename     = "web"
    serviceportname = "http"
  })
}

# --- Control plane TLS ---

resource "tls_private_key" "linkerdTrustAnchor" {
  algorithm   = "ECDSA"
  ecdsa_curve = "P256"
}

resource "tls_self_signed_cert" "linkerdTrustAnchor" {
  key_algorithm         = tls_private_key.linkerdTrustAnchor.algorithm
  private_key_pem       = tls_private_key.linkerdTrustAnchor.private_key_pem
  validity_period_hours = 17520
  is_ca_certificate     = true

  subject {
    common_name = "root.linkerd.cluster.local"
  }

  allowed_uses = [
    "cert_signing",
    "crl_signing"
  ]
}

resource "kubernetes_secret" "linkerdTrustAnchor" {
  metadata {
    name      = "linkerd-trust-anchor"
    namespace = kubernetes_namespace.linkerdNamespace.metadata.0.name
  }

  type = "kubernetes.io/tls"

  data = {
    "tls.crt" = tls_self_signed_cert.linkerdTrustAnchor.cert_pem
    "tls.key" = tls_private_key.linkerdTrustAnchor.private_key_pem
  }
}

resource "kubectl_manifest" "linkerdIdentityIssuerIssuer" {
  yaml_body  = file("linkerd-issuer.yaml")
  depends_on = [kubernetes_secret.linkerdTrustAnchor]
}

resource "kubectl_manifest" "linkerdIdentityIssuerCert" {
  yaml_body  = file("linkerd-issuer-cert.yaml")
  depends_on = [kubectl_manifest.linkerdIdentityIssuerIssuer]
}
