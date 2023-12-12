# rust-lambda-with-opentofu

### command
```shell
$ brew tap cargo-lambda/cargo-lambda
$ brew install cargo-lambda
$ cargo lambda new <project> --http-feature=apigw_rest
$ cargo lambda build --release # gen binary
$ cargo lambda deploy
```

### Docs
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