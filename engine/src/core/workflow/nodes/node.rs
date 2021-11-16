use async_trait::async_trait;
use core::fmt::Display;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use super::super::process_state::{ProcessState, ProcessStatus};

#[async_trait]
pub trait Node {
    fn prepare_input(
        &self,
        bag: &mut Value,
        result: &Value,
        actor_data: &Value,
        external_input: &Value,
    ) -> Value;

    async fn run(&self, input: Value) -> anyhow::Result<ProcessState>;
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct HelloNode;

#[async_trait]
impl Node for HelloNode {
    fn prepare_input(&self,_: &mut Value, _: &Value, _: &Value, _: &Value) -> Value {
        json!("hello world carai")
    }

    async fn run(&self, input: Value) -> anyhow::Result<ProcessState> {
        Ok(ProcessState::new(
            ProcessStatus::Finished,
            Value::Null,
            Value::Null,
        ))
    }
}
