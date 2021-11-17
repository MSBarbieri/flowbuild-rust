use super::ActivityManager;
use core::cell::RefCell;
use chrono::{DateTime, Utc};
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
    pub bag: Value,
    /// `Node` result data, this data gonna be sended to next workflow node. Value is serde parsed data
    pub result: Value,
    ///
    pub external_input: Value,
    ///
    pub actor_data: Value,
    /// result Type of Node processing
    pub status: ProcessStatus,
    pub activity_manager: Option<RefCell<ActivityManager>>
}

pub enum DataHistory {
    Bag,
    Result,
    ActorData,
    ExternalInput,
}

impl ProcessState {
    /// node Result Generator
    pub fn new(status: ProcessStatus, result: Value, external_input: Value) -> Self {
        ProcessState {
            id: Uuid::new_v4(),
            created_at: Utc::now(),
            time_elapsed: Duration::default(),
            status,
            result,
            external_input,
            actor_data: Value::Null,
            bag: Value::Null,
            engine_id: Uuid::default(),
            process_id: Uuid::default(),
            step_number: 0,
            activity_manager: None
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
    ) {
        self.time_elapsed = time_elapsed;
        self.time_elapsed = time_elapsed;
        self.engine_id = engine_id;
        self.process_id = process_id;
        self.step_number = step_number;
        self.actor_data = actor_data;
    }

    pub fn get_from(&self, data_type: DataHistory, key: &str) -> Option<&Value> {
        match data_type {
            DataHistory::Bag => self.bag.get(key),
            DataHistory::Result => self.result.get(key),
            DataHistory::ActorData => self.actor_data.get(key),
            DataHistory::ExternalInput => self.external_input.get(key),
        }
    }
}
