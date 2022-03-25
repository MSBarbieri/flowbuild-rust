use engine::{process::Process, Engine};
use serde_json::json;

#[tokio::main(flavor = "multi_thread", worker_threads = 8)]
async fn main() {
    let mut engine = Engine::new();    
}
