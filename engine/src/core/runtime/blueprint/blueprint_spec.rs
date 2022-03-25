use crate::ProcessState;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::HashMap};
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
    HashMap(Value),
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
    pub fn next(self, blueprint_spec: &BlueprintSpec, state: &ProcessState) -> Option<NodeSpec> {
        match self.next {
            Next::Text(ref text) => {
                let node = blueprint_spec.nodes.iter().find(|node| node.id.eq(text))?;
                Some(node.clone())
            }
            Next::HashMap(ref map) => {
                let node_id_result = map
                    .get(state.result.to_string())
                    .get_or_insert(map.get("default")?)
                    .to_string();
                let node = blueprint_spec
                    .nodes
                    .iter()
                    .find(|node| node.id.eq(&node_id_result))?;
                Some(node.clone())
            }
        }
    }
}
pub type EnvironmentSpec = HashMap<String, String>;

#[derive(Debug, Clone, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct BlueprintSpec {
    lanes: Vec<LaneSpec>,
    pub nodes: Vec<NodeSpec>,
    requirements: Vec<String>,
    prepare: Vec<String>,
    environment: EnvironmentSpec,
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
    pub fn get_start_node_id(&self) -> Option<String> {
        self.nodes
            .iter()
            .find(|node| node.node_type.eq("Start") || node.node_type.eq("start"))
            .map(|node_spec| node_spec.id.clone())
    }
    pub fn get_current_node_spec(&self, node_spec_id: String) -> Option<NodeSpec> {
        let node = self.nodes.iter().find(|node| node.id.eq(&node_spec_id))?;
        Some(node.clone())
    }

    pub fn get_current_lane_spec(&self, lane_spec_id: String) -> Option<LaneSpec> {
        let lane = self.lanes.iter().find(|node| node.id.eq(&lane_spec_id))?;
        Some(lane.clone())
    }

    pub fn get_parsed_requirements(&self) -> Result<HashMap<String, String>> {
        todo!();
    }

    pub fn get_parsed_environment_variables(&self) -> Result<HashMap<String, String>> {
        todo!();
    }

    pub fn validate(&self) -> Result<()> {
        self.validate_nodes()?;
        self.validate_lanes()?;
        Ok(())
    }
}
