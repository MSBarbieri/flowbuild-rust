use super::persistors::{PersistMode, PersistorProvider};
use super::threadpool::{EnginePoolnfo, EngineThreadPool};
use super::workflow::{HelloNode, Node};
use core::pin::Pin;
use serde_json::Value;
use std::collections::HashMap;
use uuid::Uuid;

pub struct Engine<'a> {
    pub engine_id: Uuid,
    pub threadpool: EngineThreadPool,
    node_hash_map: HashMap<&'a str, &'a dyn Node>,
}

impl<'a> Engine<'a> {
    pub fn new(persist_mode: PersistMode, thread_pool_info: Option<EnginePoolnfo>) -> Self {
        let node_hash_map:HashMap<&'a str, &'a dyn Node> = HashMap::new();
        node_hash_map.insert("HelloWorld", &HelloNode);
        Engine {
            engine_id: Uuid::new_v4(),
            threadpool: EngineThreadPool::new(thread_pool_info),
            node_hash_map,
        }
    }

    fn set_node(&mut self, node_name: &'a str, node_type: &'a impl Node) -> anyhow::Result<()> {
        self.node_hash_map.insert(node_name, node_type);
        Ok(())
    }

    pub async fn create_process(&mut self) -> anyhow::Result<()> {
        let uuid = Uuid::new_v4();
        let node = self.node_hash_map.get("HelloWorld").unwrap();
        let result = node
            .run(node.prepare_input(&mut Value::Null, &Value::Null, &Value::Null, &Value::Null))
            .await?;

        println!("{:?}", result);
        Ok(())
    }
}

// let mut node = self.get_node(String::from("HelloNode"));
