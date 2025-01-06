# About this app

This project uses [Cargo Lambda](https://www.cargo-lambda.info), aiming to execute scraping functionality on AWS Lambda.
Before examing this project, you need to setup your terminal to be able to run `cargo` and `cargo lambda` commands.

## Setup

1. Install [Rust](https://www.rust-lang.org/tools/install)
2. Install [Cargo Lambda](https://www.cargo-lambda.info/guide/getting-started.html)
3. (For local development) Prepare .env file with the following variables:
    - SCRAPE_SERVICE_ENDPOINT_URL, which is the endpoint URL of the scrape service (this app is currently designed to work only with [scrapeops.io API](https://proxy.scrapeops.io/v1/))
    - SCRAPE_SERVICE_API_KEY, which is the API key of the scrape service.
    - DATABASE_URL, which is the connection string to your database.
4. (For AWS Lambda) Include the following environment variables in the Lambda function:
    - SCRAPE_SERVICE_ENDPOINT_URL, which is the endpoint URL of the scrape service (this app is currently designed to work only with [scrapeops.io API](https://proxy.scrapeops.io/v1/))
    - AWS_API_KEY_SECRETS_NAME, which is the name of the secret in AWS Secrets Manager. The secret should contain SCRAPE_OPS_API_KEY, which is the API key of the scrape service.
    - AWS_DB_SECRETS_NAME, which is the name of the secret in AWS Secrets Manager. The secret should contain username, password, host, port, dbname.

## Run this app

```bash
LOCAL=true cargo lambda watch
```

When you scrape for about 10 targets, it will take some time and hot reload will highly likely terminate the process and your test will fail. In such case, you can run the following command to test the app:

```bash
LOCAL=true cargo lambda watch --ignore-changes
```

This command prevents hot reload from terminating the process. Notice that you need to run `cargo lambda watch` again to apply changes.

From another terminal, run the following command to test the app:

```bash
cargo lambda invoke --data-file fixtures/example-eventbridge-schedule.json
```

The request data file is basically a copy from [the AWS official fixture](https://github.com/awslabs/aws-lambda-rust-runtime/blob/main/lambda-events/src/fixtures/example-apigw-request.json).
You can test another API Gateway request by editing the file or copying this file and modify it.
