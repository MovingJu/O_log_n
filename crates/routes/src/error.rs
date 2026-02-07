use aide::axum::{ApiRouter, routing::get};
use axum::{
    Json,
    extract::{Query, State},
};
use log::error;
use services::AppState;

use crate::prelude::*;

pub(crate) fn get_router(state: Arc<AppState>) -> (Option<Tag>, ApiRouter) {
    (
        None,
        ApiRouter::new()
            .api_route("/log/error", get(error))
            .with_state(state)
            .with_tag("log"),
    )
}

pub(crate) async fn error(
    State(_state): State<Arc<AppState>>,
    Query(query): Query<LogQuery>,
) -> Json<ApiResponse<Empty>> {
    error!("{}, {}, {}", query.service, query.trace_id, query.message);
    Json(ApiResponse {
        code: ApiStatusCode(StatusCode::OK),
        resp: "ok".to_string(),
        data: Empty{},
    })
}
