use aide::axum::{ApiRouter, routing::post};
use axum::{Json, extract::State};
use services::{
    AppState,
    config::{Level, LogQuery},
    prelude::state::LogState,
};

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
    State(state): State<Arc<AppState>>,
    Json(query): Json<LogQueryExt>,
) -> Json<ApiResponse<Empty>> {
    let res = match query.level {
        Level::Debug => state.debug.save(query.query).await,
        Level::Info => state.info.save(query.query).await,
        Level::Warn => state.warn.save(query.query).await,
        Level::Error => state.error.save(query.query).await,
    };
    match res {
        Ok(..) => Json(ApiResponse {
            code: ApiStatusCode(StatusCode::OK),
            resp: "ok".to_string(),
            data: Empty {},
        }),
        Err(err) => Json(ApiResponse {
            code: ApiStatusCode(StatusCode::INTERNAL_SERVER_ERROR),
            resp: format!("Internal Error: {}", err),
            data: Empty,
        }),
    }
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub(crate) struct LogQueryExt {
    pub level: Level,
    pub query: LogQuery,
}
