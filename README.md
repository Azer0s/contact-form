# contact-form

A simple contact form for my portfolio site made in Rust for AWS Lambda.

## Initial Setup

```
cd deploy
./tf-init.sh
```

## Deploying

```
cd deploy
terraform apply
```

This will deploy the function to AWS Lambda, create a DynamoDB table, create the API Gateway endpoint for it, create IAM rules for the function (to allow it to write to the DynamoDB table and send emails via SES) and deploy the API Gateway stage.

The first time you build the function, you might be confronted with `Error: Provider produced inconsistent final plan`.
This is a known issue with Terraform and can be resolved by running `terraform apply` again. This should not happen when you run `terraform apply` after the first time.

## TODO
- [x] Add a terraform script
  - [x] Create a DynamoDB table
  - [x] Build the function
  - [x] Deploy the function
  - [x] Add the IAM role to the function
  - [x] Add the function to the API Gateway
- [x] Add the verification endpoint (just the GET method of the function)
