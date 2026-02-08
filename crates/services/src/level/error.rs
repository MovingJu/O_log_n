use std::path::PathBuf;

use crate::{
    config::{Expiration, Level, LogQuery},
    prelude::state::*,
};
use log::error;
use tokio::io::Result;

pub struct ErrorState {
    expiration: usize,
    path: PathBuf,
}
impl ErrorState {
    pub(crate) fn new() -> Self {
        Self {
            expiration: Expiration::Error as usize,
            path: PathBuf::from(Level::Error),
        }
    }
}
#[async_trait::async_trait]
impl LogState for ErrorState {
    async fn save(&self, jsonl: LogQuery) -> Result<()> {
        error!("{}, {}, {}", jsonl.service, jsonl.trace_id, jsonl.message);
        Ok(())
    }
    async fn read(&self) -> Result<String> {
        todo!()
    }
}
