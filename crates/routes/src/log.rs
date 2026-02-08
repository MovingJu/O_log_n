use aide::axum::{ApiRouter, routing::post};
use axum::{
    Json,
    extract::State,
};
use log::{debug, error, info, warn};
use services::AppState;

use crate::prelude::*;

pub(crate) fn get_router(state: Arc<AppState>) -> (Option<Tag>, ApiRouter) {
    (
        None,
        ApiRouter::new()
            .api_route("/log", post(log))
            .with_state(state)
            .with_tag("log"),
    )
}

pub(crate) async fn log(
    State(_state): State<Arc<AppState>>,
    Json(query): Json<LogQueryExt>,
) -> Json<ApiResponse<Empty>> {
    match query.level {
        Level::Debug => debug!("{}, {}, {}", query.query.service, query.query.trace_id, query.query.message),
        Level::Info => info!("{}, {}, {}", query.query.service, query.query.trace_id, query.query.message),
        Level::Warn => warn!("{}, {}, {}", query.query.service, query.query.trace_id, query.query.message),
        Level::Error => error!("{}, {}, {}", query.query.service, query.query.trace_id, query.query.message)
    };
    Json(ApiResponse {
        code: ApiStatusCode(StatusCode::OK),
        resp: "ok".to_string(),
        data: Empty{},
    })
}

#[derive(Deserialize, JsonSchema)]
pub(crate) struct LogQueryExt {
    pub level: Level,
    pub query: LogQuery
}
#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all="lowercase")]
pub(crate) enum Level {
    Debug,
    Info,
    Warn,
    Error
}