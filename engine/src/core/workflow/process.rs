use crate::Engine;
use chrono::{DateTime, Utc};
use core::cell::RefCell;
use serde::{Deserialize, Serialize};
use serde_json::{Value};
use uuid::Uuid;

use crate::blueprint_spec::NodeSpec;

use super::{
    blueprint_spec::BlueprintSpec,
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
    current_state: RefCell<ProcessState>,
    current_status: ProcessStatus,
}

impl Into<Process> for ProcessTemplate {
    fn into(self) -> Process {
        todo!()
    }
}

impl Into<ProcessTemplate> for Process {
    fn into(mut self) -> ProcessTemplate {
        ProcessTemplate {
            id: self.id,
            created_at: self.created_at,
            workflow_id: self.workflow.get_mut().id.clone(),
            current_state_id: self.current_state.get_mut().id.clone(),
            current_status: self.current_status,
        }
    }
}

impl Default for Process {
    fn default() -> Self {
        Self {
            id: Uuid::default(),
            created_at: Utc::now(),
            workflow: RefCell::new(Workflow::default()),
            blueprint_spec: RefCell::new(BlueprintSpec::default()),
            current_state: RefCell::new(ProcessState::new(
                ProcessStatus::Unstarted,
                Value::Null,
                Value::Null,
            )),
            current_status: ProcessStatus::Unstarted,
        }
    }
}

impl Iterator for Process {
    type Item = NodeSpec;
    fn next(&mut self) -> std::option::Option<<Self as std::iter::Iterator>::Item> {
        NodeSpec::next(
            self.blueprint_spec.get_mut(),
            self.current_state.get_mut(),
        )
    }
}

impl Process {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn run(
        &mut self,
        engine: &Engine<'_>,
        entry_input: Value,
        actor_data: Value,
    ) -> anyhow::Result<()> {
        let mut bag = Value::Null;
        let mut result = entry_input;
        let external_input = Value::Null;

        let engine_id = engine.id.clone();
        let process_id = self.id.clone();

        let mut current_step: u32 = 0;
        let mut current_state = self.current_state.clone();
        // start process 
        for mut node_spec in self.into_iter() {
            let node_type = node_spec.node_type.clone();
            if let Some(node) = engine.node_hash_map.get(&*node_type) {
                let before_processing = Utc::now();
                // Run current Node
                let input = node.prepare_input(&mut bag, &result, &actor_data, &external_input);
                let mut process_state = node.run(input).await?;
                let after_processing = Utc::now();

                // update process_state
                result = process_state.result.clone();
                current_step += 1;
                process_state.step_number = current_step.clone();
                process_state.process_id = process_id;
                process_state.time_elapsed = (before_processing - after_processing).to_std()?;
                process_state.engine_id = engine_id;

                current_state = RefCell::new(process_state.clone());

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
                    ProcessStatus::Expired => todo!(),
                };
                node_spec.id = String::from("10");
            } else {
                panic!("node dont exists");
            }
        }

        // update process after finish processing;
        self.current_state = current_state;
        self.current_status = self.current_state.get_mut().status.clone();
        Ok(())
    }
}
