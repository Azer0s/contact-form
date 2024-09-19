variable "project_name" {
  type = string
  default = "contact_me"
}

variable "function_name" {
  type = string
  default = "contact_me"
}

variable "table_name" {
  type = string
  default = "contact_form_messages"
}

variable "bucket_name" {
    type = string
    default = "contact-me-lambda-bucket"
}

variable "api_name" {
  type = string
  default = "portfolio-site-api"
}

variable "lambda_role_name" {
  type = string
  default = "contact_me_lambda_role"
}

variable "lambda_dynamodb_policy_name" {
  type = string
  default = "contact_me_lambda_dynamodb_policy"
}

variable "lambda_ses_policy_name" {
  type = string
  default = "contact_me_lambda_ses_policy"
}

variable "application_sender_name" {
  type = string
  default = "Ariel Simulevski"
}

variable "application_sender_firstname" {
    type = string
    default = "Ariel"
}

variable "application_from_email" {
    type = string
    default = "ariel@simulevski.at"
}

variable "application_from_name" {
    type = string
    default = "Ariel Simulevski"
}

variable "application_sender_email" {
    type = string
    default = "noreply@simulevski.at"
}

variable "application_email_subject" {
    type = string
    default = "Hi there!"
}

variable "application_receiver_email" {
    type = string
    default = "ariel@simulevski.at"
}