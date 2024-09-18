resource "aws_apigatewayv2_api" "main" {
  name          = "portfolio-site-api"
  protocol_type = "HTTP"
}

resource "aws_apigatewayv2_stage" "main" {
  api_id = aws_apigatewayv2_api.main.id
  name   = "main"
}

resource "aws_apigatewayv2_integration" "main" {
  api_id               = aws_apigatewayv2_api.main.id
  connection_type      = "INTERNET"
  integration_type     = "AWS_PROXY"
  integration_uri      = aws_lambda_function.main.invoke_arn
  passthrough_behavior = "WHEN_NO_MATCH"
}

resource "aws_apigatewayv2_route" "main" {
  api_id    = aws_apigatewayv2_api.main.id
  route_key = "$default"
  target    = "integrations/${aws_apigatewayv2_integration.main.id}"
}

resource "aws_lambda_permission" "lambda_permission" {
  statement_id  = "AllowContactMeLambdaInvoke"
  action        = "lambda:InvokeFunction"
  function_name = aws_lambda_function.main.function_name
  principal     = "apigateway.amazonaws.com"
  source_arn    = "${aws_apigatewayv2_api.main.execution_arn}/*"
}