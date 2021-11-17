use async_trait::async_trait;
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

#[cfg(test)]
mod NodeTest {
    use super::*;
    #[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
    pub struct HelloNode;

    #[async_trait]
    impl Node for HelloNode {
        fn prepare_input(&self, _: &mut Value, _: &Value, _: &Value, _: &Value) -> Value {
            json!("hello world carai")
        }

        async fn run(&self, input: Value) -> anyhow::Result<ProcessState> {
            println!("{}", input);
            Ok(ProcessState::new(
                ProcessStatus::Finished,
                Value::Null,
                Value::Null,
            ))
        }
    }

    #[tokio::test]
    async fn hello_world_node() {
        let input =
            HelloNode.prepare_input(&mut Value::Null, &Value::Null, &Value::Null, &Value::Null);
        let process_state_result = HelloNode.run(input).await;
        match process_state_result {
            Ok(process_state) => {
                assert_eq!(
                    process_state.status,
                    ProcessStatus::Finished
                )
            },
            _ => { panic!("error")}
        }
    }
}
