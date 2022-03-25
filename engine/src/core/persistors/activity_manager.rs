use crate::ActivityManager;
use anyhow::Result;

pub struct ActivityManagerProvider;

impl ActivityManagerProvider {
    pub fn get_activity_manager() -> Result<ActivityManager> {
        Ok(ActivityManager)
    }
}