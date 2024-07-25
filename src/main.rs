use anyhow::Result;
use std::sync::Arc;

use axum::{
    routing::{get, post},
    serve, Router,
};
use handlers::{pop_from_queue, push_to_queue};
use tokio::net::TcpListener;

mod handlers;
mod queue;

#[tokio::main]
async fn main() -> Result<()> {
    let queue = queue::Queue::new(10);

    let app = Router::new()
        .route("/queue", post(push_to_queue))
        .route("/queue", get(pop_from_queue))
        .with_state(Arc::new(queue));

    let listener = TcpListener::bind("0.0.0.0:3005").await.unwrap();
    serve(listener, app).await?;
    Ok(())
}
