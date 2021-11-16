use std::collections::HashMap;

use serde_json::Value;

pub struct LaneSpec {
    id: String,
    name: String,
    /// TODO(matheus-barbieri): implement lisp|js rule
    rule: Value,
}

pub enum Next {
    Text(String),
    HashMap(HashMap<String, String>),
}
pub struct NodeSpec {
    id: String,
    next: Next,
}

pub struct RequirementsSpec {}

pub struct PrepareSpec {}
pub struct BlueprintSpec {
    requirements: HashMap<String, String>,
    lanes: Vec<LaneSpec>,
    nodes: Vec<NodeSpec>,
}
