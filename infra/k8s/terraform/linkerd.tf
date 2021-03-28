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
  version     = "~>2.10"
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
  version     = "~>0.1"
  max_history = 5

  depends_on = [helm_release.linkerd]
}

# --- TLS ---

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
