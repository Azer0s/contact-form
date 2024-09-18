resource "aws_dynamodb_table" "main" {
  name           = "contact_form_messages"
  hash_key       = "id"
  read_capacity  = 1
  write_capacity = 1

  attribute {
    name = "id"
    type = "S"
  }
}