use crate::{blueprint::BlueprintSpec, schema::workflows};
use diesel::{Queryable, QueryableByName};
use serde_json::Value;
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};
use chrono::{NaiveDateTime,Utc, TimeZone};
use uuid::Uuid;
use crate::core::runtime::Workflow as RuntimeWorkflow;

#[derive(Debug, Queryable, PartialEq, Eq, Clone)]
pub struct Workflow {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub blueprint_spec: Value,
    pub blueprint_hash: String,
    pub version: i32,
    pub created_at: NaiveDateTime,
}

impl Into<Workflow> for RuntimeWorkflow {
    fn into(self) -> Workflow {
        let mut hasher = DefaultHasher::new();
        let spec = serde_json::to_value(&self.blueprint_spec).expect("falied to serialize");
        let blueprint_string = serde_json::to_string(&spec).expect("falied stringify");
        blueprint_string.hash(&mut hasher);
        Workflow {
            id: self.id,
            name: self.name,
            description: self.description,
            blueprint_spec: spec,
            blueprint_hash: hasher.finish().to_string(),
            version: self.version as i32,
            created_at: self.created_at.naive_utc(),
        }
    }
}

impl Into<RuntimeWorkflow> for Workflow {
    fn into(self) -> RuntimeWorkflow {
        crate::core::Workflow {
            id: self.id,
            created_at: Utc.from_local_datetime(&self.created_at).unwrap(),
            name: self.name,
            description: self.description,
            blueprint_spec: serde_json::from_value(self.blueprint_spec)
                .expect("failed to map blueprint_spec"),
            version: self.version as u32,
            published: true,
        }
    }
}

#[derive(Debug, QueryableByName)]
#[table_name = "workflows"]
pub(crate) struct QueryableWorkflow {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub blueprint_spec: Value,
    pub blueprint_hash: String,
    pub version: i32,
    pub created_at: NaiveDateTime,
}

impl Into<RuntimeWorkflow> for &QueryableWorkflow {
    fn into(self) -> RuntimeWorkflow {
        let mut hasher = DefaultHasher::new();
        let spec = serde_json::to_value(&self.blueprint_spec).expect("falied to serialize");
        let blueprint_string = serde_json::to_string(&spec).expect("falied stringify");
        blueprint_string.hash(&mut hasher);

        RuntimeWorkflow {
            id: self.id,
            name: self.name.clone(),
            description: self.description.clone(),
            blueprint_spec: serde_json::from_value(spec).expect("failed to map blueprint_spec"),
            version: self.version as u32,
            created_at: Utc.from_local_datetime(&self.created_at).unwrap(),
            published: true
        }
    }
}
#[derive(Insertable)]
#[table_name = "workflows"]
pub struct NewWorkflow {
    name: String,
    description: Option<String>,
    blueprint_spec: Value,
    blueprint_hash: String,
}

impl NewWorkflow {
    pub fn new(name: String, description: Option<String>, blueprint: BlueprintSpec) -> Self {
        let mut hasher = DefaultHasher::new();
        let spec = serde_json::to_value(&blueprint).expect("falied to serialize");
        let blueprint_string = serde_json::to_string(&spec).expect("falied stringify");
        blueprint_string.hash(&mut hasher);

        NewWorkflow {
            name,
            description,
            blueprint_spec: serde_json::to_value(&blueprint).expect("falied to serialize"),
            blueprint_hash: hasher.finish().to_string(),
        }
    }
}
