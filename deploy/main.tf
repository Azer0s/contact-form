variable "region" {
  default = "eu-central-1"
}

provider "aws" {
  region = var.region
}

resource "aws_dynamodb_table" "dynamodb_table" {
  name           = "contact_form_messages"
  hash_key       = "id"
  read_capacity  = 1
  write_capacity = 1

  attribute {
    name = "id"
    type = "S"
  }
}

