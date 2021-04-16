use super::persistors::{PersistMode, PersistorProvider};
use super::threadpool::{EnginePoolnfo, EngineThreadPool};
use std::collections::hash_map::HashMap;
use std::fmt::Debug;
use uuid::Uuid;

use crate::nodes::{HelloNode, Node};

#[derive(Debug)]
pub struct Engine {
    pub engine_id: Uuid,
    pub nodes_map: HashMap<String, Box<dyn Node>>,
    pub threadpool: EngineThreadPool,
}

impl Engine {
    pub fn new(persist_mode: PersistMode, thread_pool_info: Option<EnginePoolnfo>) -> Engine {
        let mut _nodes_map: HashMap<String, Box<dyn Node>> = HashMap::new();
        _nodes_map.insert("hello_node".to_string(), Box::new(HelloNode::new()));
        PersistorProvider::new(Some(persist_mode));
        Engine {
            engine_id: Uuid::new_v4(),
            nodes_map: _nodes_map,
            threadpool: EngineThreadPool::new(thread_pool_info),
        }
    }
    pub fn insert_node(&mut self, name: String, node: Box<dyn Node>) -> &Self {
        self.nodes_map.insert(name, node).unwrap();
        self
    }
    pub fn create_process(&mut self) -> Uuid {
        let uuid = Uuid::new_v4();
        println!("engine id in create_process {:?}", self.engine_id);
        println!("{:?}", uuid);
        let mut node = self.nodes_map.get_mut("hello_node").unwrap().clone();

        let real_node = match node.as_mut_any().downcast_mut::<HelloNode>() {
            Some(_node) => _node,
            None => panic!("fuck"),
        };
        real_node.pre_processing();
        real_node.run();
        uuid
    }
}
