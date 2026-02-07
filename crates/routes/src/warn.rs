use aide::axum::{ApiRouter, routing::get};
use axum::{
    Json,
    extract::{Query, State},
};
use log::warn;
use services::AppState;

use crate::prelude::*;

pub fn get_router(state: Arc<AppState>) -> (Option<Tag>, ApiRouter) {
    (
        None,
        ApiRouter::new()
            .api_route("/log/warn", get(warn))
            .with_state(state)
            .with_tag("log"),
    )
}

pub async fn warn(
    State(_state): State<Arc<AppState>>,
    Query(query): Query<LogQuery>,
) -> Json<ApiResponse<Empty>> {
    warn!("{}, {}, {}", query.service, query.trace_id, query.message);
    Json(ApiResponse {
        code: ApiStatusCode(StatusCode::OK),
        resp: "ok".to_string(),
        data: Empty{}
    })
}