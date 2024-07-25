use anyhow::Result;

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
}
