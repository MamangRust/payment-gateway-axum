use chrono::{DateTime, Utc};
use example_payment_gateway_axum::{
    abstract_trait::withdraw::{MockWithdrawRepositoryTrait, WithdrawRepositoryTrait},
    domain::request::withdraw::{CreateWithdrawRequest, UpdateWithdrawRequest},
    entities::withdraws,
};
use mockall::predicate::*;
use tokio;

#[tokio::test]
async fn test_find_all_withdraws() {
    let mut mock_repo = MockWithdrawRepositoryTrait::new();

    let mock_withdraw = withdraws::Model {
        withdraw_id: 1,
        user_id: 1001,
        withdraw_amount: 2000,
        withdraw_time: DateTime::<Utc>::from(Utc::now()).naive_utc(),
        created_at: Some(DateTime::<Utc>::from(Utc::now()).naive_utc()),
        updated_at: Some(DateTime::<Utc>::from(Utc::now()).naive_utc()),
    };

    mock_repo
        .expect_find_all()
        .returning(move || Ok(vec![mock_withdraw.clone()]));

    let result = mock_repo.find_all().await;

    assert!(result.is_ok());
    let withdraws = result.unwrap();
    assert_eq!(withdraws.len(), 1);
    assert_eq!(withdraws[0].withdraw_id, 1);
}

#[tokio::test]
async fn test_find_withdraw_by_id() {
    let mut mock_repo = MockWithdrawRepositoryTrait::new();

    let mock_withdraw = withdraws::Model {
        withdraw_id: 1,
        user_id: 1001,
        withdraw_amount: 2000,
        withdraw_time: DateTime::<Utc>::from(Utc::now()).naive_utc(),
        created_at: Some(DateTime::<Utc>::from(Utc::now()).naive_utc()),
        updated_at: Some(DateTime::<Utc>::from(Utc::now()).naive_utc()),
    };

    mock_repo
        .expect_find_by_id()
        .with(eq(1))
        .returning(move |_| Ok(Some(mock_withdraw.clone())));

    let result = mock_repo.find_by_id(1).await;

    assert!(result.is_ok());
    let withdraw = result.unwrap();
    assert!(withdraw.is_some());
    assert_eq!(withdraw.unwrap().withdraw_id, 1);
}

#[tokio::test]
async fn test_create_withdraw() {
    let mut mock_repo = MockWithdrawRepositoryTrait::new();

    let mock_request = CreateWithdrawRequest {
        user_id: 1001,
        withdraw_amount: 2000,
        withdraw_time: DateTime::<Utc>::from(Utc::now()),
    };

    let mock_withdraw = withdraws::Model {
        withdraw_id: 1,
        user_id: mock_request.user_id,
        withdraw_amount: mock_request.withdraw_amount,
        withdraw_time: mock_request.withdraw_time.naive_utc(),
        created_at: Some(DateTime::<Utc>::from(Utc::now()).naive_utc()),
        updated_at: Some(DateTime::<Utc>::from(Utc::now()).naive_utc()),
    };

    mock_repo
        .expect_create()
        .withf(move |input| {
            input.user_id == mock_request.user_id
                && input.withdraw_amount == mock_request.withdraw_amount
                && input.withdraw_time == mock_request.withdraw_time
        })
        .returning(move |_| Ok(mock_withdraw.clone()));

    let result = mock_repo.create(&mock_request).await;

    assert!(result.is_ok());
    let withdraw = result.unwrap();
    assert_eq!(withdraw.withdraw_id, 1);
    assert_eq!(withdraw.withdraw_amount, 2000);
}

#[tokio::test]
async fn test_update_withdraw() {
    let mut mock_repo = MockWithdrawRepositoryTrait::new();

    let mock_request = UpdateWithdrawRequest {
        user_id: 1001,
        withdraw_id: 1,
        withdraw_amount: 3000,
        withdraw_time: DateTime::<Utc>::from(Utc::now()),
    };

    let mock_withdraw = withdraws::Model {
        withdraw_id: mock_request.withdraw_id,
        user_id: mock_request.user_id,
        withdraw_amount: mock_request.withdraw_amount,
        withdraw_time: mock_request.withdraw_time.naive_utc(),
        created_at: Some(DateTime::<Utc>::from(Utc::now()).naive_utc()),
        updated_at: Some(DateTime::<Utc>::from(Utc::now()).naive_utc()),
    };

    mock_repo
        .expect_update()
        .withf(move |input| input.withdraw_id == mock_request.withdraw_id)
        .returning(move |_| Ok(mock_withdraw.clone()));

    let result = mock_repo.update(&mock_request).await;

    assert!(result.is_ok());
    let withdraw = result.unwrap();
    assert_eq!(withdraw.withdraw_id, 1);
    assert_eq!(withdraw.withdraw_amount, 3000);
}

#[tokio::test]
async fn test_delete_withdraw() {
    let mut mock_repo = MockWithdrawRepositoryTrait::new();

    mock_repo.expect_delete().with(eq(1)).returning(|_| Ok(()));

    let result = mock_repo.delete(1).await;

    assert!(result.is_ok());
}
