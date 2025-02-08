use crate::{
    domain::{
        request::transfer::{CreateTransferRequest, UpdateTransferRequest},
        response::{transfer::TransferResponse, ApiResponse},
    },
    middleware::jwt,
    state::AppState,
};
use axum::{
    extract::{Extension, Path, State},
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::{delete, get, post, put},
    Json, 
};
use serde_json::json;
use utoipa_axum::router::OpenApiRouter;
use std::sync::Arc;

#[utoipa::path(
    get,
    path = "/api/transfers",
    tag = "Transfer",
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "List of transfer", body = ApiResponse<Vec<TransferResponse>>),
        (status = 401, description = "Unauthorized", body = String),
        (status = 500, description = "Internal Server Error", body = String),
    )
)]
pub async fn get_transfers(
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    match data.di_container.transfer_service.get_transfers().await {
        Ok(response) => Ok((StatusCode::OK, Json(json!(response)))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(json!(e)))),
    }
}

#[utoipa::path(
    get,
    path = "/api/transfers/{id}",
    tag = "Topup",
security(
        ("bearer_auth" = [])
    ),
    params(
        ("id" = i32, Path, description = "Transfer ID")
    ),
    responses(
        (status = 200, description = "Transfer details", body = ApiResponse<Option<TransferResponse>>),
        (status = 401, description = "Unauthorized", body = String),
        (status = 404, description = "Topup not found", body = String),
    )
)]
pub async fn get_transfer(
    State(data): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Extension(_user_id): Extension<i64>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    match data.di_container.transfer_service.get_transfer(id).await {
        Ok(response) => Ok((StatusCode::OK, Json(json!(response)))),

        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(json!(e)))),
    }
}

#[utoipa::path(
    get,
    path = "/api/transfers/users/{id}",
    tag = "Transfer",
    security(
        ("bearer_auth" = [])
    ),
    params(
        ("id" = i32, Path, description = "Transfer ID")
    ),
    responses(
        (status = 200, description = "Topup details", body = ApiResponse<Option<Vec<TransferResponse>>>),
        (status = 401, description = "Unauthorized", body = String),
        (status = 404, description = "Topup not found", body = String),
    )
)]
pub async fn get_transfer_users(
    State(data): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Extension(_user_id): Extension<i64>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    match data
        .di_container
        .transfer_service
        .get_transfer_users(id)
        .await
    {
        Ok(response) => Ok((StatusCode::OK, Json(json!(response)))),

        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(json!(e)))),
    }
}

#[utoipa::path(
    get,
    path = "/api/transfers/user/{id}",
    tag = "Transfer",
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "List of transfer", body = ApiResponse<Option<TransferResponse>>),
        (status = 401, description = "Unauthorized", body = String),
        (status = 500, description = "Internal Server Error", body = String),
    )
)]
pub async fn get_transfer_user(
    State(data): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Extension(_user_id): Extension<i64>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    match data
        .di_container
        .transfer_service
        .get_transfer_user(id)
        .await
    {
        Ok(saldo) => Ok((StatusCode::OK, Json(json!(saldo)))),

        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(json!(e)))),
    }
}

#[utoipa::path(
    post,
    path = "/api/transfers",
    tag = "Transfer",
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "List of transfers", body = ApiResponse<TransferResponse>),
        (status = 401, description = "Unauthorized", body = String),
        (status = 500, description = "Internal Server Error", body = String),
    )
)]
pub async fn create_transfer(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CreateTransferRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    match data
        .di_container
        .transfer_service
        .create_transfer(&body)
        .await
    {
        Ok(response) => Ok((StatusCode::CREATED, Json(json!(response)))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(json!(e)))),
    }
}

#[utoipa::path(
    put,
    path = "/api/transfers/{id}",
    tag = "Transfer",
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "Update Transfer", body = ApiResponse<TransferResponse>),
        (status = 401, description = "Unauthorized", body = String),
        (status = 500, description = "Internal Server Error", body = String),
    )
)]
pub async fn update_transfer(
    State(data): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Json(mut body): Json<UpdateTransferRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    body.transfer_id = id;

    match data
        .di_container
        .transfer_service
        .update_transfer(&body)
        .await
    {
        Ok(response) => Ok((StatusCode::OK, Json(json!(response)))),

        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(json!(e)))),
    }
}

#[utoipa::path(
    delete,
    path = "/api/transfers/{id}",
    tag = "Transfer",
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "Transfer deleted successfully", body = serde_json::Value),
        (status = 401, description = "Unauthorized", body = String),
        (status = 500, description = "Internal Server Error", body = String),
    )
)]
pub async fn delete_transfer(
    State(data): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Extension(_user_id): Extension<i64>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    match data.di_container.topup_service.delete_topup(id).await {
        Ok(_) => Ok((
            StatusCode::OK,
            Json(json!({
                "status": "success",
                "message": "Transfer deleted successfully"
            })),
        )),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(json!(e)))),
    }
}

pub fn transfers_routes(app_state: Arc<AppState>) -> OpenApiRouter {
    OpenApiRouter::new()
        .route("/api/transfers", get(get_transfers))
        .route("/api/transfers/{id}", get(get_transfer))
        .route("/api/transfers/users/{id}", get(get_transfer_users))
        .route("/api/transfers/user/{id}", get(get_transfer_user))
        .route("/api/transfers", post(create_transfer))
        .route("/api/transfers/{id}", put(update_transfer))
        .route("/api/transfers/{id}", delete(delete_transfer))
        .route_layer(middleware::from_fn_with_state(app_state.clone(), jwt::auth))
        .with_state(app_state.clone())
}
