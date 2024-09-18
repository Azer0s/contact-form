locals {
  filename = "../target/lambda/${var.function_name}/bootstrap.zip"
  source = "../target/lambda/${var.function_name}/bootstrap"
}

locals {
  lambda_source_files = concat(
    [
      "../Cargo.toml",
      "../Cargo.lock",
    ],
    tolist(fileset(".", "../src/**"))
  )

  lambda_source_files_hashes = {
    for source_file in local.lambda_source_files :
    source_file => sha256(file(source_file))
  }
}

resource "terraform_data" "build_lambda" {
  triggers_replace = local.lambda_source_files_hashes

  provisioner "local-exec" {
    working_dir = ".."
    command     = <<-EOT
      cargo lambda build --release --output-format zip
    EOT
  }
}

resource "aws_s3_bucket" "contact_me_lambda_bucket" {
  bucket = "contact-me-lambda-bucket"
}

resource "aws_s3_object" "contact_me_lambda_zip" {
  depends_on = [terraform_data.build_lambda]
  bucket = aws_s3_bucket.contact_me_lambda_bucket.bucket
  key    = "bootstrap.zip"
  source = local.filename
  etag   = filemd5(local.filename)
}