use core::cell::RefCell;
use serde::{Deserialize, Serialize};
use chrono::{DateTime,Utc};
use uuid::Uuid;

use crate::{Process, blueprint_spec::BlueprintSpec};


#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Workflow {
    pub id: Uuid,
    created_at: DateTime<Utc>,
    name: String,
    description: String,
    blueprint_spec: RefCell<BlueprintSpec>,
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
            published: false,
            blueprint_spec: RefCell::new(BlueprintSpec::default()),
        }
     }
}

//static methods
impl Workflow {
    pub fn get_by_name(name: String) -> Self {
        todo!();
    }
    pub fn create_process(&self) -> Process {
        todo!()
    }
}
