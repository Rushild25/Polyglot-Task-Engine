//! High-Throughput Task Engine Entrypoint
use std::sync::Arc;
use tokio::sync::mpsc;

mod engine;
use engine::{TaskCoordinator, TaskMessage};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting Polyglot-Task-Engine runtime on port 8080...");
    let (tx, rx) = mpsc::channel::<TaskMessage>(100_000);
    let coordinator = Arc::new(TaskCoordinator::new(tx));
    coordinator.run_worker_pool(rx, 8).await;
    Ok(())
}
