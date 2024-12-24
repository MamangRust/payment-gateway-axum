use chrono::{DateTime, Utc};
use example_payment_gateway_axum::{
    abstract_trait::saldo::{MockSaldoRepositoryTrait, SaldoRepositoryTrait},
    domain::request::saldo::{CreateSaldoRequest, UpdateSaldoRequest},
    entities::saldo,
};
use mockall::predicate::*;
use tokio;

#[tokio::test]
async fn test_find_all() {
    let mut mock_repo = MockSaldoRepositoryTrait::new();
    let mock_saldo = saldo::Model {
        saldo_id: 1,
        user_id: 1,
        total_balance: 1000,
        withdraw_amount: None,
        withdraw_time: None,
        created_at: Some(DateTime::<Utc>::from(Utc::now()).naive_utc()),
        updated_at: Some(DateTime::<Utc>::from(Utc::now()).naive_utc()),
    };

    mock_repo
        .expect_find_all()
        .return_once(move || Ok(vec![mock_saldo]));

    let result = mock_repo.find_all().await;

    assert!(result.is_ok());
    let saldos = result.unwrap();
    assert_eq!(saldos.len(), 1);
    assert_eq!(saldos[0].saldo_id, 1);
    assert_eq!(saldos[0].total_balance, 1000);
}

#[tokio::test]
async fn test_find_by_id() {
    let mut mock_repo = MockSaldoRepositoryTrait::new();
    let mock_saldo = saldo::Model {
        saldo_id: 1,
        user_id: 1,
        total_balance: 1000,
        withdraw_amount: None,
        withdraw_time: None,
        created_at: Some(DateTime::<Utc>::from(Utc::now()).naive_utc()),
        updated_at: Some(DateTime::<Utc>::from(Utc::now()).naive_utc()),
    };

    mock_repo
        .expect_find_by_id()
        .with(eq(1))
        .return_once(move |_| Ok(Some(mock_saldo.clone())));

    let result = mock_repo.find_by_id(1).await;

    assert!(result.is_ok());
    let saldo = result.unwrap();
    assert!(saldo.is_some());
    assert_eq!(saldo.unwrap().saldo_id, 1);
}

#[tokio::test]
async fn test_create_saldo() {
    let mut mock_repo = MockSaldoRepositoryTrait::new();
    let mock_request = CreateSaldoRequest {
        user_id: 1,
        total_balance: 2000,
    };
    let mock_saldo = saldo::Model {
        saldo_id: 1,
        user_id: mock_request.user_id,
        total_balance: mock_request.total_balance,
        withdraw_amount: None,
        withdraw_time: None,
        created_at: Some(DateTime::<Utc>::from(Utc::now()).naive_utc()),
        updated_at: Some(DateTime::<Utc>::from(Utc::now()).naive_utc()),
    };

    mock_repo
        .expect_create()
        .with(always())
        .return_once(move |_| Ok(mock_saldo.clone()));

    let result = mock_repo.create(&mock_request).await;

    assert!(result.is_ok());
    let saldo = result.unwrap();
    assert_eq!(saldo.user_id, 1);
    assert_eq!(saldo.total_balance, 2000);
}

#[tokio::test]
async fn test_update_saldo() {
    let mut mock_repo = MockSaldoRepositoryTrait::new();

    let mock_request = UpdateSaldoRequest {
        saldo_id: 1,
        user_id: 1,
        total_balance: 3000,
        withdraw_amount: Some(500),
        withdraw_time: Some(Utc::now().naive_utc()),
    };

    let mock_saldo = saldo::Model {
        saldo_id: mock_request.saldo_id,
        user_id: mock_request.user_id,
        total_balance: mock_request.total_balance,
        withdraw_amount: mock_request.withdraw_amount,
        withdraw_time: mock_request.withdraw_time,
        created_at: Some(DateTime::<Utc>::from(Utc::now()).naive_utc()),
        updated_at: Some(DateTime::<Utc>::from(Utc::now()).naive_utc()),
    };

    mock_repo
        .expect_update()
        .with(always())
        .return_once(move |_| Ok(mock_saldo.clone()));

    let result = mock_repo.update(&mock_request).await;

    assert!(result.is_ok());
    let saldo = result.unwrap();
    assert_eq!(saldo.saldo_id, 1);
    assert_eq!(saldo.user_id, 1);
    assert_eq!(saldo.total_balance, 3000);
    assert_eq!(saldo.withdraw_amount, Some(500));
}

#[tokio::test]
async fn test_update_balance() {
    let mut mock_repo = MockSaldoRepositoryTrait::new();
    let mock_request = UpdateSaldoRequest {
        saldo_id: 1,
        user_id: 1,
        total_balance: 3000,
        withdraw_amount: Some(500),
        withdraw_time: Some(DateTime::<Utc>::from(Utc::now()).naive_utc()),
    };

    let mock_saldo = saldo::Model {
        saldo_id: mock_request.saldo_id,
        user_id: mock_request.user_id,
        total_balance: mock_request.total_balance,
        withdraw_amount: mock_request.withdraw_amount,
        withdraw_time: mock_request.withdraw_time,
        created_at: Some(DateTime::<Utc>::from(Utc::now()).naive_utc()),
        updated_at: Some(DateTime::<Utc>::from(Utc::now()).naive_utc()),
    };

    mock_repo
        .expect_update()
        .with(always())
        .return_once(move |_| Ok(mock_saldo.clone()));

    let result = mock_repo.update(&mock_request).await;

    assert!(result.is_ok());
    let saldo = result.unwrap();
    assert_eq!(saldo.saldo_id, 1);
    assert_eq!(saldo.total_balance, 3000);
    assert_eq!(saldo.withdraw_amount, Some(500));
}

#[tokio::test]
async fn test_delete_saldo() {
    let mut mock_repo = MockSaldoRepositoryTrait::new();

    mock_repo
        .expect_delete()
        .with(eq(1))
        .return_once(|_| Ok(()));

    let result = mock_repo.delete(1).await;

    assert!(result.is_ok());
}
