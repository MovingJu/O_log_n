use std::path::PathBuf;

use crate::{
    config::{Expiration, Level, LogQuery},
    prelude::state::*,
};
use log::debug;
use tokio::io::Result;

pub struct DebugState {
    expiration: usize,
    path: PathBuf,
}
impl DebugState {
    pub(crate) fn new() -> Self {
        Self {
            expiration: Expiration::Debug as usize,
            path: PathBuf::from(Level::Debug),
        }
    }
}
#[async_trait::async_trait]
impl LogState for DebugState {
    async fn save(&self, jsonl: LogQuery) -> Result<()> {
        debug!("{}, {}, {}", jsonl.service, jsonl.trace_id, jsonl.message);
        Ok(())
    }
    async fn read(&self) -> Result<String> {
        todo!()
    }
}
