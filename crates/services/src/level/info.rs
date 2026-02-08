use std::path::PathBuf;

use crate::{
    config::{Expiration, Level, LogQuery},
    prelude::state::*,
};
use log::info;
use tokio::io::Result;

pub struct InfoState {
    expiration: usize,
    path: PathBuf,
}
impl InfoState {
    pub(crate) fn new() -> Self {
        Self {
            expiration: Expiration::Info as usize,
            path: PathBuf::from(Level::Info),
        }
    }
}
#[async_trait::async_trait]
impl LogState for InfoState {
    async fn save(&self, jsonl: LogQuery) -> Result<()> {
        info!("{}, {}, {}", jsonl.service, jsonl.trace_id, jsonl.message);
        Ok(())
    }
    async fn read(&self) -> Result<String> {
        todo!()
    }
}
