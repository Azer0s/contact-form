resource "aws_lambda_function" "main" {
  function_name = var.function_name
  role          = aws_iam_role.iam_for_lambda.arn
  handler       = "rust.handler"
  runtime       = "provided.al2023"
  architectures = ["x86_64"]
  s3_bucket     = aws_s3_bucket.contact_me_lambda_bucket.bucket
  s3_key        = aws_s3_object.contact_me_lambda_zip.key
  source_code_hash = filemd5(local.filename)

  environment {
    variables = {
      TABLE_NAME       = aws_dynamodb_table.main.name
      SENDER_NAME      = var.application_sender_name
      SENDER_FIRSTNAME = var.application_sender_firstname
      SENDER_EMAIL     = var.application_sender_email
      FROM_NAME        = var.application_from_name
      FROM_EMAIL       = var.application_from_email
      EMAIL_SUBJECT    = var.application_email_subject
    }
  }
}
