use http::{Response, StatusCode};
use vercel_runtime::{run, Body, Error, Request, Response as VercelResponse};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(_req: Request) -> Result<VercelResponse<Body>, Error> {
    let response = Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/html")
        .body(Body::Text("M-Pesa API is running".to_string()))?;
        
    Ok(response)
}