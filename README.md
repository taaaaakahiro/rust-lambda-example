# rust-lambda-with-opentofu

### init
```shell
$ brew tap cargo-lambda/cargo-lambda
$ brew install cargo-lambda
$ cargo lambda new <project> --http-feature=apigw_rest
```

### deploy & destroy
```shell
$ export AWS_ACCESS_KEY_ID=xxx
$ export AWS_SECRET_ACCESS_KEY=xxx
$ cargo lambda build --release #gen binary
$ cargo lambda build --release --arm64
$ cargo lambda deploy #deploy
$ aws lambda delete-function --function-name <lambda-func-name> #destroy
```

### Docs
- https://github.com/awslabs/aws-lambda-rust-runtime
- setup
  - https://developer.mamezou-tech.com/blogs/2023/03/19/aws-lambda-with-rust/ 
- AWS
  - https://docs.aws.amazon.com/ja_jp/lambda/latest/dg/rust-handler.html
- deploy by CDK
  - https://dev.classmethod.jp/articles/rust-cdk-lambda/
- deploy by terraform
  - https://engineering.nifty.co.jp/blog/21191
- response streaming
  - https://dev.classmethod.jp/articles/rust-response-streaming/