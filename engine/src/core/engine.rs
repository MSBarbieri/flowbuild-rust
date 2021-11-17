use super::workflow::Node;
use std::collections::HashMap;
use uuid::Uuid;

pub struct Engine<'a> {
    pub id: Uuid,
    pub node_hash_map: HashMap<&'a str, &'a dyn Node>,
}

impl<'a> Engine<'a> {
    pub fn new() -> Self {
        Engine {
            id: Uuid::new_v4(),
            node_hash_map: HashMap::new(),
        }
    }
}
