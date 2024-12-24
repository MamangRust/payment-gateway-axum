use example_payment_gateway_axum::{
    abstract_trait::jwt::{JwtServiceTrait, MockJwtServiceTrait},
    utils::errors::AppError,
};
use mockall::predicate::*;

#[tokio::test]
async fn test_generate_token() {
    let mut mock_jwt_service = MockJwtServiceTrait::new();

    let user_id = 1;

    let mock_token = "mock_token".to_string();
    let mock_token_clone = mock_token.clone();

    mock_jwt_service
        .expect_generate_token()
        .with(eq(user_id))
        .returning(move |_| Ok(mock_token.clone()));

    let result = mock_jwt_service.generate_token(user_id);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), mock_token_clone);
}

#[tokio::test]
async fn test_verify_token_success() {
    let mut mock_jwt_service = MockJwtServiceTrait::new();

    let token = "valid_token";
    let user_id = 1;

    mock_jwt_service
        .expect_verify_token()
        .with(eq(token))
        .returning(move |_| Ok(user_id));

    let result = mock_jwt_service.verify_token(token);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), user_id);
}

#[tokio::test]
async fn test_verify_token_expired() {
    let mut mock_jwt_service = MockJwtServiceTrait::new();

    let token = "expired_token";

    mock_jwt_service
        .expect_verify_token()
        .with(eq(token))
        .returning(move |_| Err(AppError::TokenExpiredError));

    let result = mock_jwt_service.verify_token(token);

    assert!(result.is_err());
    if let AppError::TokenExpiredError = result.unwrap_err() {
        assert!(true);
    } else {
        assert!(false, "Expected AppError::TokenExpiredError");
    }
}
