use std::error::Error;

use axum::{routing::get, Router, Server};

#[tokio::main]
pub async fn run() -> Result<(), Box<dyn Error>> {
    let app = Router::new().route("/", get(root));

    Server::bind(&"0.0.0.0:80".parse()?)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn root() -> &'static str {
    "Hello 👋"
}
