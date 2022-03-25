use engine::blueprint::BlueprintSpec;
use serde_json::json;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let blueprint_string = include_str!("blueprint.json");
    let json_value: BlueprintSpec = serde_json::from_str(blueprint_string)?;
    let node = json_value.get_start_node_id();
    println!("{:?}", node);
    Ok(())
}
