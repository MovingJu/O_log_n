/// All *State structs MUST impl these traits
pub mod state {
    use crate::config::LogQuery;
    use tokio::io::Result;

    #[async_trait::async_trait]
    pub trait LogState {
        async fn save(&self, jsonl: LogQuery) -> Result<()>;
        async fn read(&self) -> Result<String>;
    }
}
