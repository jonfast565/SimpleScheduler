use serde_json::serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
struct Job {
    name: String,
    run_every_seconds: i64,
}