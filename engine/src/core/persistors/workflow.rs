use crate::models::workflow::{NewWorkflow, QueryableWorkflow, Workflow};
use crate::runtime::Workflow as RuntimeWorkflow;
use crate::Provider;
use anyhow::Result;
use diesel::{prelude::*, sql_query};
pub struct WorkflowProvider;

impl Provider for WorkflowProvider {}

impl WorkflowProvider {
    pub fn get_workflows(&self) -> Result<Vec<RuntimeWorkflow>> {
        let conn = self.get_persistor().get_pool().get()?;

        let w = sql_query(
            "
            SELECT * 
            FROM workflows as w1
            where version = (
                select max(version) from workflows as w2 
                where w1.name = w2.name
            )",
        )
        .load::<QueryableWorkflow>(&conn)?;
        println!("{:?}", w);
        let w = w
            .iter()
            .map(|w| w.into())
            .collect();
        Ok(w)
    }

    pub fn get_workflows_by_name(&self, workflow_name: &str) -> Result<Vec<RuntimeWorkflow>> {
        let conn = self.get_persistor().get_pool().get()?;
        let query = format!(
            "
            SELECT * 
            FROM workflows as w1
            where w1.name = '{}'
            AND version = (
                select max(version) from workflows as w2 
                where w1.name = w2.name
            )",
            workflow_name
        );

        let w = sql_query(query).load::<QueryableWorkflow>(&conn)?;
        let w = w
            .iter()
            .map(|w| w.into())
            .collect();
        Ok(w)
    }

    pub fn add_workflow(&self, workflow: NewWorkflow) -> Result<RuntimeWorkflow> {
        use crate::schema::workflows;
        let conn = self.get_persistor().get_pool().get()?;
        let result: Workflow = diesel::insert_into(workflows::table)
            .values(workflow)
            .get_result(&conn)
            .expect("Error saving new post");
        Ok(result.into())
    }
}

#[cfg(test)]
mod workflow_persistor_tests {
    use serde_json::json;

    use crate::{
        blueprint::BlueprintSpec,
        models::workflow::{self, NewWorkflow},
    };

    use super::WorkflowProvider;

    #[tokio::test]
    pub async fn get_workflows_test() {
        let provider = WorkflowProvider;
        provider.get_workflows().expect("failed to get workflows");
    }

    #[tokio::test]
    pub async fn add_workflow_test() {
        let provider = WorkflowProvider;
        let new_workflow = NewWorkflow::new(
            "test_workflow".to_string(),
            Some("test description".to_string()),
            BlueprintSpec::new(),
        );
        let workflow = provider
            .add_workflow(new_workflow)
            .expect("workflow not created correctly");
        let workflows = provider
            .get_workflows_by_name("test_workflow")
            .expect("failed to get workflow");

        let db_workflow = workflows.get(0);
        if let Some(w) = db_workflow {
            assert_eq!(&workflow, w);
        } else {
            panic!("add workflow error");
        }
    }
}
