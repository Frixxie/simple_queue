use anyhow::Result;

use crossbeam::queue::SegQueue;
use tokio::sync::Mutex;

pub struct Queue {
    queue: Mutex<Vec<Vec<u8>>>,
}

impl Queue {
    pub fn new(len: usize) -> Self {
        let vec = Vec::with_capacity(len);
        Self {
            queue: Mutex::new(vec),
        }
    }

    pub async fn enqueue(&self, item: &[u8]) -> Result<()> {
        self.queue.lock().await.push(item.to_vec());
        Ok(())
    }

    pub async fn dequeue(&self) -> Option<Vec<u8>> {
        self.queue.lock().await.pop()
    }

    pub async fn length(&self) -> usize {
        self.queue.lock().await.len()
    }
}

pub struct CrossBeamQueue {
    queue: SegQueue<Vec<u8>>,
}

impl CrossBeamQueue {
    pub fn new(_len: usize) -> Self {
        Self {
            queue: SegQueue::new(),
        }
    }

    pub async fn enqueue(&self, item: &[u8]) -> Result<()> {
        self.queue.push(item.to_vec());
        Ok(())
    }

    pub async fn dequeue(&self) -> Option<Vec<u8>> {
        self.queue.pop()
    }

    pub async fn length(&self) -> usize {
        self.queue.len()
    }
}
