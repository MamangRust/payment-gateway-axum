use chrono::{DateTime, Utc};
use example_payment_gateway_axum::{
    abstract_trait::topup::{MockTopupRepositoryTrait, TopupRepositoryTrait},
    domain::request::topup::{CreateTopupRequest, UpdateTopupRequest},
    entities::topups,
};
use mockall::predicate::*;
use tokio;

#[tokio::test]
async fn test_find_all_topups() {
    let mut mock_repo = MockTopupRepositoryTrait::new();

    let mock_topup = topups::Model {
        topup_id: 1,
        user_id: 1,
        topup_no: "TOP12345".to_string(),
        topup_amount: 1000,
        topup_method: "Bank Transfer".to_string(),
        topup_time: DateTime::from_timestamp(1_634_944_800, 0)
            .unwrap()
            .naive_utc(),
        created_at: None,
        updated_at: None,
    };

    mock_repo
        .expect_find_all()
        .return_once(move || Ok(vec![mock_topup.clone()]));

    let result = mock_repo.find_all().await;

    assert!(result.is_ok());
    let topups = result.unwrap();
    assert_eq!(topups.len(), 1);
    assert_eq!(topups[0].topup_no, "TOP12345");
    assert_eq!(topups[0].topup_amount, 1000);
}

#[tokio::test]
async fn test_find_topup_by_id() {
    let mut mock_repo = MockTopupRepositoryTrait::new();

    let mock_topup = topups::Model {
        topup_id: 1,
        user_id: 1,
        topup_no: "TOP12345".to_string(),
        topup_amount: 1000,
        topup_method: "Bank Transfer".to_string(),
        topup_time: DateTime::<Utc>::from(Utc::now()).naive_utc(),
        created_at: None,
        updated_at: None,
    };

    mock_repo
        .expect_find_by_id()
        .with(eq(1))
        .return_once(move |_| Ok(Some(mock_topup.clone())));

    let result = mock_repo.find_by_id(1).await;

    assert!(result.is_ok());
    let topup = result.unwrap();
    assert!(topup.is_some());
    let topup = topup.unwrap();
    assert_eq!(topup.topup_id, 1);
    assert_eq!(topup.topup_no, "TOP12345");
}

#[tokio::test]
async fn test_create_topup() {
    let mut mock_repo = MockTopupRepositoryTrait::new();

    let mock_request = CreateTopupRequest {
        user_id: 1,
        topup_no: "TOP12345".to_string(),
        topup_amount: 2000,
        topup_method: "Credit Card".to_string(),
    };

    let mock_topup = topups::Model {
        topup_id: 1,
        user_id: mock_request.user_id,
        topup_no: mock_request.topup_no.clone(),
        topup_amount: mock_request.topup_amount,
        topup_method: mock_request.topup_method.clone(),
        topup_time: DateTime::<Utc>::from(Utc::now()).naive_utc(),
        created_at: None,
        updated_at: None,
    };

    mock_repo
        .expect_create()
        .with(always())
        .return_once(move |_| Ok(mock_topup.clone()));

    let result = mock_repo.create(&mock_request).await;

    assert!(result.is_ok());
    let topup = result.unwrap();
    assert_eq!(topup.topup_no, "TOP12345");
    assert_eq!(topup.topup_amount, 2000);
}

#[tokio::test]
async fn test_update_topup() {
    let mut mock_repo = MockTopupRepositoryTrait::new();

    let mock_request = UpdateTopupRequest {
        user_id: 1,
        topup_id: 1,
        topup_amount: 3000,
        topup_method: "Bank Transfer".to_string(),
    };

    let mock_topup = topups::Model {
        topup_id: mock_request.topup_id,
        user_id: mock_request.user_id,
        topup_no: "TOP12345".to_string(),
        topup_amount: mock_request.topup_amount,
        topup_method: mock_request.topup_method.clone(),
        topup_time: DateTime::<Utc>::from(Utc::now()).naive_utc(),
        created_at: None,
        updated_at: None,
    };

    mock_repo
        .expect_update()
        .with(always())
        .return_once(move |_| Ok(mock_topup.clone()));

    let result = mock_repo.update(&mock_request).await;

    assert!(result.is_ok());
    let topup = result.unwrap();
    assert_eq!(topup.topup_amount, 3000);
    assert_eq!(topup.topup_method, "Bank Transfer");
}

#[tokio::test]
async fn test_delete_topup() {
    let mut mock_repo = MockTopupRepositoryTrait::new();

    mock_repo
        .expect_delete()
        .with(eq(1))
        .return_once(|_| Ok(()));

    let result = mock_repo.delete(1).await;

    assert!(result.is_ok());
}