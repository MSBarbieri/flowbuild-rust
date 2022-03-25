use super::ActivityManager;
use chrono::{DateTime, Utc};
use core::cell::RefCell;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::time::Duration;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ProcessStatus {
    Unstarted,
    Waiting,
    Running,
    Finished,
    Error(String),
    Interrupted(String),
    Pending,
    Forbidden,
    Delegated,
    Expired,
}

/// this state is a representation of any type of `Node` result, this state gonna be serialized and persisted
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct ProcessState {
    pub id: Uuid,

    pub created_at: DateTime<Utc>,
    /// Node processing time duration
    pub time_elapsed: Duration,
    /// id ref of process
    pub process_id: Uuid,
    /// id ref of engine
    pub engine_id: Uuid,
    /// count of how many states this process have
    pub step_number: u32,
    /// data for process history
    pub node_id: String,
    /// process storage
    pub bag: Value,
    /// `Node` result data, this data gonna be sended to next workflow node. Value is serde parsed data
    pub result: Value,
    ///
    pub external_input: Value,
    ///
    pub actor_data: Value,
    /// result Type of Node processing
    pub status: ProcessStatus,
    pub activity_manager: Option<RefCell<ActivityManager>>,
}

impl Default for ProcessState {
    fn default() -> Self {
        Self {
            id: Default::default(),
            created_at: Utc::now(),
            time_elapsed: Default::default(),
            process_id: Default::default(),
            engine_id: Default::default(),
            step_number: Default::default(),
            node_id: Default::default(),
            bag: Default::default(),
            result: Default::default(),
            external_input: Default::default(),
            actor_data: Default::default(),
            status: ProcessStatus::Unstarted,
            activity_manager: Default::default(),
        }
    }
}

pub enum DataHistory {
    Bag,
    Result,
    ActorData,
    ExternalInput,
}

impl ProcessState {
    /// node Result Generator
    pub fn new(
        node_id: String,
        actor_data: Value,
        status: ProcessStatus,
        bag: Value,
        engine_id: Uuid
    ) -> Self {
        ProcessState {
            id: Uuid::new_v4(),
            created_at: Utc::now(),
            time_elapsed: Duration::default(),
            status,
            result: Value::Null,
            external_input: Value::Null,
            node_id: node_id,
            actor_data: actor_data,
            bag: bag,
            engine_id: engine_id,
            process_id: Uuid::default(),
            step_number: 0,
            activity_manager: None,
        }
    }

    pub fn from_node_result(
        status: ProcessStatus,
        result: Value,
        activity_manager: Option<RefCell<ActivityManager>>,
        bag: Value,
    ) -> Self {
        ProcessState {
            id: Uuid::new_v4(),
            created_at: Utc::now(),
            time_elapsed: Duration::default(),
            status,
            result,
            external_input: Value::Null,
            node_id: String::default(),
            actor_data: Value::Null,
            bag: bag,
            engine_id: Uuid::default(),
            process_id: Uuid::default(),
            step_number: 0,
            activity_manager,
        }
    }

    /// the process data must set this value after the state is created;
    pub fn set_process_data(
        &mut self,
        time_elapsed: Duration,
        engine_id: Uuid,
        process_id: Uuid,
        step_number: u32,
        actor_data: Value,
        node_id: String,
    ) {
        self.node_id = node_id;
        self.time_elapsed = time_elapsed;
        self.time_elapsed = time_elapsed;
        self.engine_id = engine_id;
        self.process_id = process_id;
        self.step_number = step_number;
        self.actor_data = actor_data;
    }
}
