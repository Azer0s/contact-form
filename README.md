# contact-form

A simple contact form for my portfolio site made in Rust for AWS Lambda.

```
cargo lambda build --release
cargo lambda deploy --region eu-central-1 --enable-function-url 
```

This will also generate an IAM role for the function. This role needs to be able to PutItem to the DynamoDB table and be able to send emails via SES.

## TODO
- [x] Add a terraform script
  - [x] Create a DynamoDB table
  - [ ] Build the function
  - [ ] Deploy the function
  - [ ] Add the IAM role to the function
  - [ ] Add the function to the API Gateway