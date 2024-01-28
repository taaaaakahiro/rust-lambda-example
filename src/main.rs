use lambda_http::{run, service_fn, Body, Error, Request, Response};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct TestEvent {
    name: String,
}

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    // リクエストからデータを Vec<u8> として取得
    let body_bytes = event.body().to_vec();

    // Vec<u8> を文字列に変換
    let body_str = String::from_utf8_lossy(&body_bytes);

    // JSON 文字列をデシリアライズ
    let test_event: TestEvent = serde_json::from_str(&body_str)?;

    // デシリアライズしたデータを利用して処理を行う
    let message = format!("Hello {}, this is an AWS Lambda HTTP request", test_event.name);

    // レスポンスを構築
    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(Body::from(message))
        .map_err(Box::new)?;

    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
