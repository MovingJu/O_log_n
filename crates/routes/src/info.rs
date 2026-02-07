use aide::axum::{ApiRouter, routing::get};
use axum::{
    Json,
    extract::{Query, State},
};
// use log::info;
use log::info;
use services::AppState;

use crate::prelude::*;

/// # get_router
/// Adds route easily in `main.rs` file.
pub fn get_router(state: Arc<AppState>) -> (Option<Tag>, ApiRouter) {
    (
        None,
        ApiRouter::new()
            .api_route("/info", get(info))
            .with_state(state)
            .with_prefix("/log")
            .with_tag("log"),
    )
}

pub async fn info(
    State(_state): State<Arc<AppState>>,
    Query(query): Query<InfoQuery>,
) -> Json<ApiResponse<InfoResponse>> {
    info!("{}, {}, {}", query.service, query.trace_id, query.message);
    Json(ApiResponse {
        code: ApiStatusCode(StatusCode::OK),
        resp: "ok".to_string(),
        data: InfoResponse {},
    })
}
#[derive(Deserialize, JsonSchema)]
pub struct InfoQuery {
    service: String,
    message: String,
    trace_id: String,
}
#[derive(Serialize, JsonSchema)]
pub struct InfoResponse {}
