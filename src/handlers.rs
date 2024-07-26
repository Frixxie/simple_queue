use std::sync::Arc;

use axum::{body::Bytes, extract::State, response::IntoResponse};

use crate::queue::Queue;

pub async fn push_to_queue(State(queue): State<Arc<Queue>>, body: Bytes) -> impl IntoResponse {
    let item = body.to_vec();
    match queue.enqueue(&item).await {
        Ok(_) => "Item added to queue",
        Err(_) => "Failed to add item to queue",
    }
}

pub async fn pop_from_queue(State(queue): State<Arc<Queue>>) -> impl IntoResponse {
    match queue.dequeue().await {
        Some(item) => item,
        None => "Queue is empty".to_string().into_bytes(),
    }
}

pub async fn get_queue_length(State(queue): State<Arc<Queue>>) -> impl IntoResponse {
    format!("{}", queue.length().await)
}
