use std::{borrow::BorrowMut, cell::RefCell, sync::Arc};

use crate::{Engine, process_state};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use super::{
    blueprint::{NodeSpec,BlueprintSpec},
    process_state::{ProcessState, ProcessStatus},
    workflow::Workflow,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessTemplate {
    id: Uuid,
    created_at: DateTime<Utc>,
    workflow_id: Uuid,
    current_state_id: Uuid,
    current_status: ProcessStatus,
}

#[derive(Debug, Clone)]
pub struct Process {
    pub id: Uuid,
    created_at: DateTime<Utc>,
    workflow: RefCell<Workflow>,
    blueprint_spec: RefCell<BlueprintSpec>,
    pub current_state: RefCell<ProcessState>,
    current_status: ProcessStatus,
}

impl Process {
    pub fn new( 
        engine: Arc<Engine>,
        workflow: RefCell<Workflow>,
        blueprint_spec: RefCell<BlueprintSpec>,
        mut process_state: ProcessState
    ) -> Self {
        let id = Uuid::new_v4();
        process_state.process_id = id;
        Process {
            id,
            created_at: Utc::now(),
            workflow,
            blueprint_spec,
            current_state: RefCell::new(process_state),
            current_status: ProcessStatus::Unstarted,
        }
    }

    fn next_node(&mut self) -> Option<NodeSpec> {
        let blueprint_spec = self.blueprint_spec.borrow();
        let process_state = self.current_state.borrow();
        blueprint_spec.get_current_node_spec(process_state.node_id.to_string())

    }

    pub async fn run(
        &mut self,
        engine: &Engine,
        entry_input: Value,
        actor_data: Value,
    ) -> anyhow::Result<()> {
        let mut bag = self.current_state.borrow_mut().bag.clone();
        let mut result = entry_input;
        let external_input = Value::Null;

        let engine_id = engine.id.clone();
        let process_id = self.id.clone();


        // start process
        while let Some(node_spec) = self.next_node() {
            let node_type = node_spec.node_type.clone();
            if let Some(node) = engine.node_hash_map.get(&node_type) {
                let before_processing = Utc::now();
                // Run current Node
                let input = node.prepare_input(&mut bag, &result, &actor_data, &external_input);
                let mut process_state = node.run(input,bag.clone()).await?;
                let after_processing = Utc::now();

                // update process_state
                result = process_state.result.clone();
                process_state.set_process_data(
                    (after_processing - before_processing).to_std()?,
                    engine_id,
                    process_id,
                    self.current_state.borrow().step_number + 1,
                    actor_data.clone(),
                    node_spec.id.clone(),
                );
                
                // activity_manager processing
                if process_state.activity_manager.is_some() {
                    todo!("activity_manager processing");
                }
                match process_state.status {
                    ProcessStatus::Unstarted => {}
                    ProcessStatus::Waiting => todo!(),
                    ProcessStatus::Running => todo!(),
                    ProcessStatus::Finished => todo!(),
                    ProcessStatus::Error(_) => todo!(),
                    ProcessStatus::Interrupted(_) => todo!(),
                    ProcessStatus::Pending => todo!(),
                    ProcessStatus::Forbidden => todo!(),
                    ProcessStatus::Delegated => todo!(),
                    ProcessStatus::Expired => break,
                };

                self.current_state.replace(process_state);

                
            } else {
                panic!("node dont exists");
            }
        }

        Ok(())
    }
}
