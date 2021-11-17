use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use super::ProcessState;
use serde_json::Value;
#[derive(Debug, Clone, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct LaneSpec {
    id: String,
    name: String,
    /// TODO(matheus-barbieri): implement lisp|js rule
    rule: Value,
}
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Next {
    Text(String),
    HashMap(HashMap<String, String>),
}
impl Default for Next {
    fn default() -> Next {
        Next::Text(String::default())
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct NodeSpec {
    pub id: String,
    pub next: Next,
    #[serde(rename = "type")]
    pub node_type: String,
}
impl NodeSpec {
    pub fn next(blueprint: &BlueprintSpec, state: &ProcessState)-> Option<NodeSpec> {
        todo!("discover next node")
    }
}
#[derive(Debug, Clone, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct RequirementsSpec {}
#[derive(Debug, Clone, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct PrepareSpec {}
#[derive(Debug, Clone, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct BlueprintSpec {
    requirements: HashMap<String, String>,
    lanes: Vec<LaneSpec>,
    nodes: Vec<NodeSpec>,
    prepare: PrepareSpec,
    environment: HashMap<String, String>,
}

impl BlueprintSpec {
    pub fn new() -> Self {
        BlueprintSpec::default()
    }

    pub fn validate_lanes(&self) -> Result<()> {
        todo!();
    }

    pub fn validate_nodes(&self) -> Result<()> {
        todo!();
    }

    pub fn get_current_node(&self, id: String) -> Option<NodeSpec> {
        todo!();
    }

    pub fn get_current_lane(&self, id: String) -> Option<LaneSpec> {
        todo!();
    }

    pub fn get_parsed_requirements(&self) -> Result<HashMap<String, String>> {
        todo!();
    }

    pub fn get_parsed_environment_variables(&self) -> Result<HashMap<String, String>> {
        todo!();
    }
}

#[cfg(test)]
mod BlueprintSpecTests {
    use super::*;
    #[test]
    fn print() {
        println!("{:?}", BlueprintSpec::new());
    }
}
