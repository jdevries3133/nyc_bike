terraform {

  backend "s3" {
    bucket = "my-sites-terraform-remote-state"
    key    = "recipe_front"
    region = "us-east-2"
  }

  required_providers {
    kubernetes = {
      source  = "hashicorp/kubernetes"
      version = ">= 2.8.0"
    }

  }
}

provider "kubernetes" {
  config_path = "~/.kube/config"
}

data "external" "git_describe" {
  program = ["sh", "-c", "echo '{\"output\": \"'\"$(git describe --tags)\"'\"}'"]
}


module "container-deployment" {
  source  = "jdevries3133/container-deployment/kubernetes"
  version = "0.3.0"

  app_name = "recipe-front"
  container = "jdevries3133/recipe_front:${data.external.git_describe.result.output}"
  domain = "bike.jackdevries.com"
}
