use super::runtime::Node;
use std::collections::HashMap;
use uuid::Uuid;

pub struct Engine {
    pub id: Uuid,
    pub node_hash_map: HashMap<String, Box<dyn Node>>,
}

impl Engine {
    pub fn new() -> Self {
        let mut engine = Engine {
            id: Uuid::new_v4(),
            node_hash_map: HashMap::new(),
        };

        //TODO: add default nodes to engine
        engine
    }

    pub fn isert_node<T: Node>(&mut self, node_name: String, node: T) {
        self.node_hash_map.insert(node_name, Box::new(node));
    }
}
