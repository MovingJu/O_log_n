pub mod config;
mod level;
pub mod prelude;
use level::*;

pub struct AppState {
    pub debug: debug::DebugState,
    pub info: info::InfoState,
    pub warn: warn::WarnState,
    pub error: error::ErrorState,
}
impl AppState {
    pub fn new() -> Self {
        Self {
            debug: debug::DebugState::new(),
            info: info::InfoState::new(),
            warn: warn::WarnState::new(),
            error: error::ErrorState::new(),
        }
    }
}
