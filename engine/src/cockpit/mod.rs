use crate::{persistors::workflow::WorkflowProvider, ActivityManager, Engine, Result, Workflow};

pub struct Cockpit;

impl Cockpit {
    pub fn new() -> Self {
        Cockpit
    }

    pub fn get_activity_manager_by_actor(&self) -> Result<ActivityManager> {
        todo!()
    }

    pub fn get_workflows(&self) -> Result<Vec<Workflow>> {
        WorkflowProvider.get_workflows()
    }

    pub fn get_workflows_by_name(&self, name: &str) -> Result<Vec<Workflow>> {
        WorkflowProvider.get_workflows_by_name(name)
    }
}
