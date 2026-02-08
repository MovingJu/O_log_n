use aide::axum::{ApiRouter, routing::get};
use axum::{
    Json,
    extract::{Query, State},
};
use services::{AppState, config::LogQuery, prelude::state::LogState};

use crate::prelude::*;

pub(crate) fn get_router(state: Arc<AppState>) -> (Option<Tag>, ApiRouter) {
    (
        None,
        ApiRouter::new()
            .api_route("/log/info", get(info))
            .with_state(state)
            .with_tag("log"),
    )
}

pub(crate) async fn info(
    State(state): State<Arc<AppState>>,
    Query(query): Query<LogQuery>,
) -> Json<ApiResponse<Empty>> {
    match state.info.save(query).await {
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
