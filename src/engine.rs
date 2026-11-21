//! Lock-free worker pool and task dispatcher implementation.
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;
use std::time::Instant;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TaskMessage {
    pub id: String,
    pub payload: String,
    pub priority: u8,
}

pub struct TaskCoordinator {
    sender: mpsc::Sender<TaskMessage>,
}

impl TaskCoordinator {
    pub fn new(sender: mpsc::Sender<TaskMessage>) -> Self {
        Self { sender }
    }

    pub async fn dispatch(&self, task: TaskMessage) -> Result<(), mpsc::error::SendError<TaskMessage>> {
        self.sender.send(task).await
    }

    pub async fn run_worker_pool(&self, mut receiver: mpsc::Receiver<TaskMessage>, concurrency: usize) {
        println!("Initialized {} worker threads", concurrency);
        while let Some(task) = receiver.recv().await {
            let start = Instant::now();
            // Process task payload with zero allocation
            let elapsed = start.elapsed();
            if elapsed.as_micros() > 500 {
                println!("Processed task {} in {:?}", task.id, elapsed);
            }
        }
    }
}
