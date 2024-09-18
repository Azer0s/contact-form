locals {
  filename = "../target/lambda/${var.function_name}/bootstrap.zip"
}

resource "aws_s3_bucket" "contact_me_lambda_bucket" {
  bucket = "contact-me-lambda-bucket"
}

resource "aws_s3_object" "contact_me_lambda_zip" {
  bucket = aws_s3_bucket.contact_me_lambda_bucket.bucket
  key    = "bootstrap.zip"
  source = local.filename
}