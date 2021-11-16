use serde::{Deserialize, Serialize};
use chrono::{DateTime,Utc};
use uuid::Uuid;


#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Workflow {
    pub id: Uuid,
    created_at: DateTime<Utc>,
    name: String,
    description: String,
    version: Uuid,
    published: bool,
}

impl Default for Workflow {
    
    fn default() -> Self { 
        Workflow {
            id: Uuid::new_v4(),
            name: String::default(),
            created_at: Utc::now(),
            description: String::default(),
            version: Uuid::new_v4(),
            published: false
        }
     }
}

//static methods
impl Workflow {
    fn get_by_name(name: String) -> Self {
        todo!();
    }
}

// instance methods
impl Workflow {
    fn create_process(&self) {
        todo!()
    }
}
