use std::sync::Arc;

use crate::{
    domain::{
        request::auth::{LoginRequest, RegisterRequest},
        response::{user::UserResponse, ApiResponse, ErrorResponse},
    },
    middleware::jwt,
    state::AppState,
};
use axum::{
    extract::State,
    http::StatusCode,
    middleware,
    response::IntoResponse,
    Extension, Json,
    routing::{get, post}
};
use serde_json::{json, Value};
use utoipa_axum::router::OpenApiRouter;

#[utoipa::path(
    get,
    path = "/api/healthchecker",
    tag = "Auth",
    responses(
        (status = 200, description = "Health check successful", body = Value)
    )
)]
pub async fn health_checker_handler() -> impl IntoResponse {
    const MESSAGE: &str = "JWT Authentication in Rust using Axum, Postgres, and SQLX";
    Json(serde_json::json!({ "status": "success", "message": MESSAGE }))
}

#[utoipa::path(
    post,
    path = "/api/auth/register",
    tag = "Auth",
    request_body = RegisterRequest,
    responses(
        (status = 200, description = "User registered successfully", body = ApiResponse<UserResponse>),
        (status = 400, description = "Bad request", body = ErrorResponse)
    )
)]
pub async fn register_user_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<RegisterRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    match data.di_container.auth_service.register_user(&body).await {
        Ok(response) => Ok((StatusCode::OK, Json(json!(response)))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(json!(e)))),
    }
}

#[utoipa::path(
    post,
    path = "/api/auth/login",
    tag = "Auth",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "User logged in successfully", body = ApiResponse<String>),
        (status = 401, description = "Unauthorized", body = ErrorResponse)
    )
)]
pub async fn login_user_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<LoginRequest>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    match data.di_container.auth_service.login_user(&body).await {
        Ok(response) => Ok((StatusCode::OK, Json(json!(response)))),
        Err(e) => Err((StatusCode::UNAUTHORIZED, Json(json!(e)))),
    }
}

#[utoipa::path(
    get,
    path = "/api/users/me",
    tag = "Auth",
    security(
        ("bearer_auth" = [])
    ),
    responses(
        (status = 200, description = "User retrieved successfully", body = ApiResponse<Option<UserResponse>>),
        (status = 401, description = "Unauthorized", body = ErrorResponse)
    )
)]
pub async fn get_me_handler(
    State(data): State<Arc<AppState>>,
    Extension(user_id): Extension<i64>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    match data
        .di_container
        .user_service
        .find_by_id(user_id as i32)
        .await
    {
        Ok(user) => Ok((StatusCode::OK, Json(json!(user)))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(json!(e)))),
    }
}

pub fn auth_routes(app_state: Arc<AppState>) -> OpenApiRouter {
    OpenApiRouter::new()
        .route("/api/healthchecker", get(health_checker_handler))
        .route("/api/auth/register", post(register_user_handler))
        .route("/api/auth/login", post(login_user_handler))
        .route(
            "/api/users/me",
            get(get_me_handler)
            .route_layer(middleware::from_fn_with_state(app_state.clone(), jwt::auth)).with_state(app_state.clone())
        )
        .with_state(app_state)
}