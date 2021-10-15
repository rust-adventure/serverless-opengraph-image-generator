use aws_lambda_events::event::apigw::{
    ApiGatewayProxyRequest, ApiGatewayProxyResponse,
};
use http::{HeaderMap, StatusCode};
use lambda_runtime::{handler_fn, Context, Error};
use serde_json::{json, Value};

const width: u32 = 1024;
const height: u32 = 512;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let handler_fn = handler_fn(handler);
    lambda_runtime::run(handler_fn).await?;
    Ok(())
}

async fn handler(
    event: Value,
    _: Context,
) -> Result<ApiGatewayProxyResponse, Error> {
    let title = &event["queryStringParameters"]["title"].as_str().or(Some("Rust Adventure Dynamic Image Serverless Function Test"));
    let subtitle = &event["queryStringParameters"]
        ["subtitle"]
        .as_str()
        .or(Some("Dynamic OpenGraph Image"));

    let resp = ApiGatewayProxyResponse {
        status_code: 200,
        headers: HeaderMap::new(),
        multi_value_headers: HeaderMap::new(),
        body: Some(format!(
            r#"<html><head>
        <meta name="description" content="Rust Adventure" />
        <meta name="twitter:card" content="summary_large_image" />
        <meta name="twitter:creator" content="@chrisbiscardi" />
        <meta property="og:type" content="website" />
        <meta property="og:description" content="Rust Adventure" />
        <meta
          property="og:image"
          content="https://rust-opengraph.netlify.app/.netlify/functions/generic/?title={}&subtitle={}"
        />
        <meta property="og:image:width" content="1024" />
        <meta property="og:image:height" content="512" />
        <meta property="og:site_name" content="Rust Adventure" />
        <meta property="author" content="Chris Biscardi" />
        <meta
          property="keywords"
          content="rust, rustlang, adventure, programming, rustlings, concepts, learn rust, learn"
        />
        <title>Learn to build reliable and efficient software in Rust</title>
        <meta
          property="og:title"
          content="Learn to build reliable and efficient software in Rust"
        />
        </head>
        <body></body></html>
       "#,
            "some title",
            "subtitle"
        ).into()),
        is_base64_encoded: Some(false),
    };
    Ok(resp)
}
