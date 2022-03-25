use async_trait::async_trait;
use serde_json::Value;

use super::super::process_state::{ProcessState, ProcessStatus};

#[async_trait]
pub trait Node: 'static {
    fn prepare_input(
        &self,
        bag: &mut Value,
        result: &Value,
        actor_data: &Value,
        external_input: &Value,
    ) -> Value;

    async fn run(&self, input: Value, bag: Value) -> anyhow::Result<ProcessState>;
}

#[cfg(test)]
mod node_test {
    use serde_json::json;

    use super::*;
    #[derive(Debug, Clone, Eq, PartialEq)]
    pub struct HelloNode;

    #[async_trait]
    impl Node for HelloNode {
        fn prepare_input(&self, _: &mut Value, _: &Value, _: &Value, _: &Value) -> Value {
            json!("hello world carai")
        }

        async fn run(&self, input: Value, bag: Value) -> anyhow::Result<ProcessState> {
            println!("{}", input);
            Ok(ProcessState::from_node_result(
                ProcessStatus::Running,
                Value::Null,
                None,
                bag
            ))
        }
    }

    #[tokio::test]
    async fn hello_world_node() {
        let input =
            HelloNode.prepare_input(&mut Value::Null, &Value::Null, &Value::Null, &Value::Null);
        let process_state_result = HelloNode.run(input,Value::Null).await;
        match process_state_result {
            Ok(process_state) => {
                assert_eq!(process_state.status, ProcessStatus::Running)
            }
            _ => {
                panic!("error")
            }
        }
    }

    #[derive(Debug, Clone, Eq, PartialEq)]
    pub struct HelloNode2;

    #[async_trait]
    impl Node for HelloNode2 {
        fn prepare_input(&self, _: &mut Value, _: &Value, _: &Value, _: &Value) -> Value {
            json!("hello world carai")
        }

        async fn run(&self, input: Value, bag: Value) -> anyhow::Result<ProcessState> {
            println!("{}", input);
            Ok(ProcessState::from_node_result(
                ProcessStatus::Running,
                Value::Null,
                None,
                bag,
            ))
        }
    }
}
