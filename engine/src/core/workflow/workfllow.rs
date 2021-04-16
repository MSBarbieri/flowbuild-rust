use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use uuid::Uuid;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
pub struct Workflow {
    id: Uuid,
    created_at: SystemTime,
    name: String,
    description: String,
    vesion: Uuid,
    published: Boolean,
}

//static methods
impl Workflow {
    fn get_by_name(name: String) -> Self {
        Workflow::default();
    }
}

// instance methods
impl Workflow {
    fn create_process(&self) {}
}
