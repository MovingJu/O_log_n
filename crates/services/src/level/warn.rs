use std::path::PathBuf;

use crate::{
    config::{Expiration, Level, LogQuery},
    prelude::state::*,
};
use log::warn;
use tokio::io::Result;

pub struct WarnState {
    expiration: usize,
    path: PathBuf,
}
impl WarnState {
    pub(crate) fn new() -> Self {
        Self {
            expiration: Expiration::Warn as usize,
            path: PathBuf::from(Level::Warn),
        }
    }
}
#[async_trait::async_trait]
impl LogState for WarnState {
    async fn save(&self, jsonl: LogQuery) -> Result<()> {
        warn!("{}, {}, {}", jsonl.service, jsonl.trace_id, jsonl.message);
        Ok(())
    }
    async fn read(&self) -> Result<String> {
        todo!()
    }
}
