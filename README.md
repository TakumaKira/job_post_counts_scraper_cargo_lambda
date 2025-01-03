# About this app

This project uses [Cargo Lambda](https://www.cargo-lambda.info), aiming to execute scraping functionality on AWS Lambda.
Before examing this project, you need to setup your terminal to be able to run `cargo` and `cargo lambda` commands.

## Setup

1. Install [Rust](https://www.rust-lang.org/tools/install)
2. Install [Cargo Lambda](https://www.cargo-lambda.info/guide/getting-started.html)

## Run this app

```bash
cargo lambda watch
```

From another terminal, run the following command to test the app:

```bash
cargo lambda invoke --data-file fixtures/example-eventbridge-schedule.json
```

The request data file is basically a copy from [the AWS official fixture](https://github.com/awslabs/aws-lambda-rust-runtime/blob/main/lambda-events/src/fixtures/example-apigw-request.json).
You can test another API Gateway request by editing the file or copying this file and modify it.
