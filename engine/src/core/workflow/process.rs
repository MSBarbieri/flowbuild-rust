use core::cell::RefCell;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use super::{
    process_state::{ProcessState, ProcessStatus},
    workflow::{BlueprintSpec, Workflow},
};

pub struct ProcessTemplate {
    id: Uuid,
    created_at: DateTime<Utc>,
    workflow_id: Uuid,
    current_state_id: Uuid,
    currrent_status: ProcessStatus,
}

pub struct Process {
    pub id: Uuid,
    created_at: DateTime<Utc>,
    workflow: Option<RefCell<Workflow>>,
    blueprint_spec: Option<RefCell<BlueprintSpec>>,
    current_state: Option<RefCell<ProcessState>>,
    currrent_status: ProcessStatus,
}

impl Into<Process> for ProcessTemplate {
    fn into(self) -> Process {
        todo!()
    }
}

impl Into<ProcessTemplate> for Process {
    fn into(self) -> ProcessTemplate {
        ProcessTemplate {
            id: self.id,
            created_at: self.created_at,
            workflow_id: self.workflow.unwrap().get_mut().id.clone(),
            current_state_id: self.current_state.unwrap().get_mut().id.clone(),
            currrent_status: self.currrent_status,
        }
    }
}
