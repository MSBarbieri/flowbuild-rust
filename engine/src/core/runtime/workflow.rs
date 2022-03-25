use anyhow::{Result,anyhow};
use chrono::{DateTime, Utc};
use core::cell::RefCell;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;
use uuid::Uuid;

use crate::{blueprint::BlueprintSpec, Engine, Process, ProcessState, ProcessStatus};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Workflow {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub name: String,
    pub description: Option<String>,
    pub blueprint_spec: RefCell<BlueprintSpec>,
    pub version: u32,
    pub published: bool,
}

//static methods
impl Workflow {
    pub fn get_by_name(name: String) -> Self {
        todo!();
    }

    pub fn create_process(
        self,
        engine: Arc<Engine>,
        actor_data: Value,
        start_input: Value,
    ) -> Result<Process> {
        let blueprint = BlueprintSpec::new();
        blueprint.validate()?;

        if let Some(node_id) = blueprint.get_start_node_id() {
            let blueprint = RefCell::new(blueprint);
            let process_state = ProcessState::new(
                node_id,
                actor_data,
                ProcessStatus::Unstarted,
                start_input,
                engine.id,
            );
            let process = Process::new(engine, RefCell::new(self), blueprint, process_state);
            Ok(process)
        }else {
            Err(anyhow!("No start node found"))
        }
    }
}
