terraform {

  backend "s3" {
    bucket = "my-sites-terraform-remote-state"
    key    = "nyc_bike_backend"
    region = "us-east-2"
  }

  required_providers {
    kubernetes = {
      source  = "hashicorp/kubernetes"
      version = ">= 2.7.1"
    }
    helm = {
      source  = "hashicorp/helm"
      version = ">= 2.4.1"
    }

  }
}

provider "kubernetes" {
  config_path = "~/.kube/config"
}

provider "helm" {
  kubernetes {
    config_path = "~/.kube/config"
  }
}

resource "random_password" "contact_inquiry_secret" {
  length  = 48
  special = false
}

data "external" "git_describe" {
  program = ["sh", "-c", "echo '{\"output\": \"'\"$(git describe --tags)\"'\"}'"]
}

module "basic-deployment" {
  source  = "jdevries3133/basic-deployment/kubernetes"
  version = "0.2.0"

  app_name  = "nyc-bike-backend"
  container = "jdevries3133/nyc_bike_backend:${data.external.git_describe.result.output}"
  domain    = "bike-app-back.jackdevries.com"
}
