use std::sync::Arc;

use chrono::{DateTime, Utc};
use example_payment_gateway_axum::{
    abstract_trait::{
        saldo::{MockSaldoRepositoryTrait, SaldoServiceTrait},
        user::{MockUserRepositoryTrait, UserRepositoryTrait},
    },
    domain::request::saldo::{CreateSaldoRequest, UpdateSaldoRequest},
    entities::{saldo, users, withdraws},
    services::{saldo::SaldoService, withdraw::WithdrawService},
};
use mockall::predicate;

#[tokio::test]
async fn test_get_saldos() {
    let mut mock_saldo_repo = MockSaldoRepositoryTrait::new();
    let mut mock_user_repo = MockUserRepositoryTrait::new();

    let mock_saldos = vec![
        saldo::Model {
            saldo_id: 1,
            user_id: 1,
            total_balance: 100000,
            withdraw_amount: None,
            withdraw_time: None,
            created_at: Some(DateTime::<Utc>::from(Utc::now()).naive_utc()),
            updated_at: Some(DateTime::<Utc>::from(Utc::now()).naive_utc()),
        },
        saldo::Model {
            saldo_id: 2,
            user_id: 1,
            total_balance: 200000,
            withdraw_amount: None,
            withdraw_time: None,
            created_at: Some(DateTime::<Utc>::from(Utc::now()).naive_utc()),
            updated_at: Some(DateTime::<Utc>::from(Utc::now()).naive_utc()),
        },
    ];

    mock_saldo_repo
        .expect_find_all()
        .return_once(move || Ok(mock_saldos.clone()));

    let service = SaldoService::new(Arc::new(mock_user_repo), Arc::new(mock_saldo_repo));

    let result = service.get_saldos().await;

    assert!(result.is_ok());

    let response = result.unwrap();

    assert_eq!(response.status, "success");
    assert_eq!(response.message, "Saldos retrieved successfully");

    let data = response.data;

    assert_eq!(data.len(), 2);

    assert_eq!(data[0].id, 1);
    assert_eq!(data[0].user_id, 1);
    assert_eq!(data[0].total_balance, 100000);
    assert_eq!(data[0].withdraw_amount, None);
    assert_eq!(data[0].withdraw_time, None);
}

#[tokio::test]
async fn test_find_by_id_success() {
    let mut mock_saldo_repo = MockSaldoRepositoryTrait::new();
    let mut mock_user_repo = MockUserRepositoryTrait::new();

    let mock_saldo = saldo::Model {
        saldo_id: 1,
        user_id: 1,
        total_balance: 100000,
        withdraw_amount: None,
        withdraw_time: None,
        created_at: Some(DateTime::<Utc>::from(Utc::now()).naive_utc()),
        updated_at: Some(DateTime::<Utc>::from(Utc::now()).naive_utc()),
    };

    mock_saldo_repo
        .expect_find_by_id()
        .with(predicate::eq(1))
        .return_once(move |_| Ok(Some(mock_saldo.clone())));

    let service = SaldoService::new(Arc::new(mock_user_repo), Arc::new(mock_saldo_repo));

    let result = service.get_saldo(1).await;

    assert!(result.is_ok());

    let response = result.unwrap();

    assert_eq!(response.status, "success");
    assert_eq!(response.message, "Saldo retrieved successfully");

    let data = response.data.unwrap();

    assert_eq!(data.id, 1);
    assert_eq!(data.user_id, 1);
    assert_eq!(data.total_balance, 100000);
    assert_eq!(data.withdraw_amount, None);
    assert_eq!(data.withdraw_time, None);
}

#[tokio::test]
async fn test_find_by_id_not_found() {
    let mut mock_saldo_repo = MockSaldoRepositoryTrait::new();
    let mut mock_user_repo = MockUserRepositoryTrait::new();

    mock_saldo_repo
        .expect_find_by_id()
        .with(predicate::eq(1))
        .return_once(move |_| Ok(None));

    let service = SaldoService::new(Arc::new(mock_user_repo), Arc::new(mock_saldo_repo));

    let result = service.get_saldo(1).await;

    assert!(result.is_err());

    let error = result.unwrap_err();

    assert_eq!(error.status, "error");
    assert!(error.message.contains("Saldo with id 1 not found"));
}

#[tokio::test]
async fn test_get_saldo_users() {
    let mut mock_saldo_repo = MockSaldoRepositoryTrait::new();
    let mut mock_user_repo = MockUserRepositoryTrait::new();

    let user_id = 1;

    mock_user_repo
        .expect_find_by_id()
        .with(predicate::eq(user_id))
        .return_once(move |_| {
            Ok(Some(users::Model {
                user_id,
                firstname: "John".to_string(),
                lastname: "Doe".to_string(),
                email: "john.doe@example.com".to_string(),
                password: "hashed_password".to_string(),
                noc_transfer: "12345".to_string(),
                created_at: Some(Utc::now().naive_utc()),
                updated_at: Some(Utc::now().naive_utc()),
            }))
        });

    let mock_saldos = vec![
        Some(saldo::Model {
            saldo_id: 1,
            user_id,
            total_balance: 100000,
            withdraw_amount: None,
            withdraw_time: None,
            created_at: Some(Utc::now().naive_utc()),
            updated_at: Some(Utc::now().naive_utc()),
        }),
        Some(saldo::Model {
            saldo_id: 2,
            user_id,
            total_balance: 200000,
            withdraw_amount: Some(50000),
            withdraw_time: Some(Utc::now().naive_utc()),
            created_at: Some(Utc::now().naive_utc()),
            updated_at: Some(Utc::now().naive_utc()),
        }),
    ];

    mock_saldo_repo
        .expect_find_by_users_id()
        .with(predicate::eq(user_id))
        .return_once(move |_| Ok(mock_saldos.clone()));

    let service = SaldoService::new(Arc::new(mock_user_repo), Arc::new(mock_saldo_repo));

    let result = service.get_saldo_users(user_id).await;

    assert!(result.is_ok());

    let response = result.unwrap();

    assert_eq!(response.status, "success");
    assert_eq!(response.message, "Success");

    let data = response.data.unwrap();
    assert_eq!(data.len(), 2);
    assert_eq!(data[0].id, 1);
    assert_eq!(data[0].total_balance, 100000);
    assert_eq!(data[1].total_balance, 200000);
    assert_eq!(data[1].withdraw_amount, Some(50000));
}

#[tokio::test]
async fn test_get_saldo_users_no_saldos() {
    let mut mock_saldo_repo = MockSaldoRepositoryTrait::new();
    let mut mock_user_repo = MockUserRepositoryTrait::new();

    let user_id = 1;

    mock_user_repo
        .expect_find_by_id()
        .with(predicate::eq(user_id))
        .return_once(move |_| {
            Ok(Some(users::Model {
                user_id,
                firstname: "John".to_string(),
                lastname: "Doe".to_string(),
                email: "john.doe@example.com".to_string(),
                password: "hashed_password".to_string(),
                noc_transfer: "12345".to_string(),
                created_at: Some(Utc::now().naive_utc()),
                updated_at: Some(Utc::now().naive_utc()),
            }))
        });

    mock_saldo_repo
        .expect_find_by_users_id()
        .with(predicate::eq(user_id))
        .return_once(move |_| Ok(vec![]));

    let service = SaldoService::new(Arc::new(mock_user_repo), Arc::new(mock_saldo_repo));

    let result = service.get_saldo_users(user_id).await;

    assert!(result.is_ok());

    let response = result.unwrap();

    assert_eq!(response.status, "success");
    assert_eq!(
        response.message,
        format!("No saldo found for user with id {}", user_id)
    );

    assert!(response.data.is_none());
}

#[tokio::test]
async fn test_get_saldo_user_success() {
    let mut mock_user_repo = MockUserRepositoryTrait::new();
    let mut mock_saldo_repo = MockSaldoRepositoryTrait::new();

    let user_id = 1;

    // Mock data user
    mock_user_repo
        .expect_find_by_id()
        .with(predicate::eq(user_id))
        .return_once(move |_| {
            Ok(Some(users::Model {
                user_id,
                firstname: "John".to_string(),
                lastname: "Doe".to_string(),
                email: "john.doe@example.com".to_string(),
                password: "hashed_password".to_string(),
                noc_transfer: "12345".to_string(),
                created_at: Some(Utc::now().naive_utc()),
                updated_at: Some(Utc::now().naive_utc()),
            }))
        });

    // Mock data saldo
    let mock_saldo = saldo::Model {
        saldo_id: 1,
        user_id,
        total_balance: 100000,
        withdraw_amount: None,
        withdraw_time: None,
        created_at: Some(Utc::now().naive_utc()),
        updated_at: Some(Utc::now().naive_utc()),
    };

    let mock_saldo_clone = mock_saldo.clone();

    mock_saldo_repo
        .expect_find_by_users_id()
        .with(predicate::eq(user_id))
        .return_once(move |_| Ok(vec![Some(mock_saldo_clone)]));

    let service = SaldoService::new(Arc::new(mock_user_repo), Arc::new(mock_saldo_repo));

    let result = service.get_saldo_user(user_id).await;

    assert!(result.is_ok());

    let response = result.unwrap();

    assert_eq!(response.status, "success");
    assert_eq!(response.message, "Success");

    let data = response.data.unwrap();

    assert_eq!(data.id, mock_saldo.saldo_id);
    assert_eq!(data.user_id, mock_saldo.user_id);
    assert_eq!(data.total_balance, mock_saldo.total_balance);
    assert_eq!(data.withdraw_amount, mock_saldo.withdraw_amount.clone());
}

#[tokio::test]
async fn test_get_saldo_user_not_found() {
    let mut mock_user_repo = MockUserRepositoryTrait::new();
    let mut mock_saldo_repo = MockSaldoRepositoryTrait::new();

    let user_id = 1;

    mock_user_repo
        .expect_find_by_id()
        .with(predicate::eq(user_id))
        .return_once(move |_| {
            Ok(Some(users::Model {
                user_id,
                firstname: "John".to_string(),
                lastname: "Doe".to_string(),
                email: "john.doe@example.com".to_string(),
                password: "hashed_password".to_string(),
                noc_transfer: "12345".to_string(),
                created_at: Some(Utc::now().naive_utc()),
                updated_at: Some(Utc::now().naive_utc()),
            }))
        });

    mock_saldo_repo
        .expect_find_by_users_id()
        .with(predicate::eq(user_id))
        .return_once(move |_| Ok(vec![]));

    let service = SaldoService::new(Arc::new(mock_user_repo), Arc::new(mock_saldo_repo));

    let result = service.get_saldo_user(user_id).await;

    assert!(result.is_ok());

    let response = result.unwrap();

    assert_eq!(response.status, "success");
    assert_eq!(
        response.message,
        format!("No saldo found for user with id {}", user_id)
    );
    assert!(response.data.is_none());
}

#[tokio::test]
async fn test_create_saldo_success() {
    let mut mock_user_repo = MockUserRepositoryTrait::new();
    let mut mock_saldo_repo = MockSaldoRepositoryTrait::new();

    let user_id = 1;
    let input = CreateSaldoRequest {
        user_id,
        total_balance: 50000,
    };

    let mock_saldo = saldo::Model {
        saldo_id: 1,
        user_id,
        total_balance: input.total_balance,
        withdraw_amount: None,
        withdraw_time: None,
        created_at: Some(Utc::now().naive_utc()),
        updated_at: Some(Utc::now().naive_utc()),
    };

    let mock_saldo_clone = mock_saldo.clone();
    mock_user_repo
        .expect_find_by_id()
        .with(predicate::eq(user_id))
        .return_once(move |_| {
            Ok(Some(users::Model {
                user_id,
                firstname: "John".to_string(),
                lastname: "Doe".to_string(),
                email: "john.doe@example.com".to_string(),
                password: "hashed_password".to_string(),
                noc_transfer: "12345".to_string(),
                created_at: Some(Utc::now().naive_utc()),
                updated_at: Some(Utc::now().naive_utc()),
            }))
        });

    mock_saldo_repo
        .expect_create()
        .with(predicate::eq(input.clone()))
        .return_once(move |_| Ok(mock_saldo_clone));

    let service = SaldoService::new(Arc::new(mock_user_repo), Arc::new(mock_saldo_repo));

    let result = service.create_saldo(&input).await;

    assert!(result.is_ok());

    let response = result.unwrap();

    assert_eq!(response.status, "success");
    assert_eq!(response.message, "Saldo created successfully");
    assert_eq!(response.data.id, mock_saldo.saldo_id);
    assert_eq!(response.data.user_id, mock_saldo.user_id);
    assert_eq!(response.data.total_balance, mock_saldo.total_balance);
}

#[tokio::test]
async fn test_create_saldo_validation_error() {
    let mock_user_repo = MockUserRepositoryTrait::new(); // Tidak diperlukan mock behavior karena validasi gagal lebih awal
    let mock_saldo_repo = MockSaldoRepositoryTrait::new();

    let input = CreateSaldoRequest {
        user_id: 1,
        total_balance: -5000,
    };

    let service = SaldoService::new(Arc::new(mock_user_repo), Arc::new(mock_saldo_repo));

    let result = service.create_saldo(&input).await;

    assert!(result.is_err());

    let error = result.unwrap_err();

    assert_eq!(error.status, "Error Validation");
    assert_eq!(
        error.message,
        "total balance must be greater than or equal to 50000"
    );
}

#[tokio::test]
async fn test_update_saldo_success() {
    let mut mock_user_repo = MockUserRepositoryTrait::new();
    let mut mock_saldo_repo = MockSaldoRepositoryTrait::new();

    let user_id = 1;
    let saldo_id = 1;
    let input = UpdateSaldoRequest {
        saldo_id,
        user_id,
        total_balance: 1000000,
        withdraw_amount: Some(50000),
        withdraw_time: Some(Utc::now().naive_utc()),
    };

    let mock_saldo = saldo::Model {
        saldo_id,
        user_id,
        total_balance: 1000000,
        withdraw_amount: input.withdraw_amount,
        withdraw_time: input.withdraw_time,
        created_at: Some(Utc::now().naive_utc()),
        updated_at: Some(Utc::now().naive_utc()),
    };

    let updated_mock_saldo = saldo::Model {
        saldo_id,
        user_id,
        total_balance: input.total_balance,
        withdraw_amount: None,
        withdraw_time: None,
        created_at: mock_saldo.created_at,
        updated_at: Some(Utc::now().naive_utc()),
    };

    let updated_mock_saldo_clone = updated_mock_saldo.clone();

    mock_user_repo
        .expect_find_by_id()
        .with(predicate::eq(user_id))
        .return_once(move |_| {
            Ok(Some(users::Model {
                user_id,
                firstname: "John".to_string(),
                lastname: "Doe".to_string(),
                email: "john.doe@example.com".to_string(),
                password: "hashed_password".to_string(),
                noc_transfer: "12345".to_string(),
                created_at: Some(Utc::now().naive_utc()),
                updated_at: Some(Utc::now().naive_utc()),
            }))
        });

    mock_saldo_repo
        .expect_find_by_id()
        .with(predicate::eq(saldo_id))
        .return_once(move |_| Ok(Some(mock_saldo.clone())));

    mock_saldo_repo
        .expect_update()
        .with(predicate::eq(input.clone()))
        .return_once(move |_| Ok(updated_mock_saldo.clone()));

    let service = SaldoService::new(Arc::new(mock_user_repo), Arc::new(mock_saldo_repo));

    let result = service.update_saldo(&input).await;

    let response = result.unwrap();

    let data = response.data.unwrap();

    assert_eq!(response.status, "success");
    assert_eq!(response.message, "Saldo updated successfully");
    assert_eq!(data.id, updated_mock_saldo_clone.saldo_id);
    assert_eq!(data.user_id, updated_mock_saldo_clone.user_id);
    assert_eq!(data.total_balance, updated_mock_saldo_clone.total_balance);
}

#[tokio::test]
async fn test_update_saldo_validation_error() {
    let mock_user_repo = MockUserRepositoryTrait::new();
    let mock_saldo_repo = MockSaldoRepositoryTrait::new();

    let input = UpdateSaldoRequest {
        saldo_id: 0,
        user_id: 1,
        total_balance: -100,
        withdraw_amount: None,
        withdraw_time: None,
    };

    let service = SaldoService::new(Arc::new(mock_user_repo), Arc::new(mock_saldo_repo));

    let result = service.update_saldo(&input).await;

    assert!(result.is_err());

    let error_response = result.unwrap_err();

    println!("Error: {:?}", error_response);

    assert_eq!(error_response.status, "Error Validation");
    assert!(error_response
        .message
        .contains("Saldo ID must be greater than 0"));
}

#[tokio::test]
async fn test_delete_saldo_success() {
    let mut mock_user_repo = MockUserRepositoryTrait::new();
    let mut mock_saldo_repo = MockSaldoRepositoryTrait::new();

    let user_id = 1;
    let saldo_id = 1;

    mock_user_repo
        .expect_find_by_id()
        .with(predicate::eq(user_id))
        .return_once(move |_| {
            Ok(Some(users::Model {
                user_id,
                firstname: "John".to_string(),
                lastname: "Doe".to_string(),
                email: "john.doe@example.com".to_string(),
                password: "hashed_password".to_string(),
                noc_transfer: "12345".to_string(),
                created_at: Some(Utc::now().naive_utc()),
                updated_at: Some(Utc::now().naive_utc()),
            }))
        });

    mock_saldo_repo
        .expect_find_by_user_id()
        .with(predicate::eq(user_id))
        .return_once(move |_| {
            Ok(Some(saldo::Model {
                saldo_id,
                user_id,
                total_balance: 100000,
                withdraw_amount: None,
                withdraw_time: None,
                created_at: Some(Utc::now().naive_utc()),
                updated_at: Some(Utc::now().naive_utc()),
            }))
        });

    mock_saldo_repo
        .expect_delete()
        .with(predicate::eq(saldo_id))
        .return_once(move |_| Ok(()));

    let service = SaldoService::new(Arc::new(mock_user_repo), Arc::new(mock_saldo_repo));

    let result = service.delete_saldo(user_id).await;

    assert!(result.is_ok());

    let response = result.unwrap();

    assert_eq!(response.status, "success");
    assert_eq!(response.message, "Saldo deleted successfully");
}

#[tokio::test]
async fn test_delete_saldo_not_found() {
    let mut mock_user_repo = MockUserRepositoryTrait::new();
    let mut mock_saldo_repo = MockSaldoRepositoryTrait::new();

    let user_id = 1;

    mock_user_repo
        .expect_find_by_id()
        .with(predicate::eq(user_id))
        .return_once(move |_| {
            Ok(Some(users::Model {
                user_id,
                firstname: "John".to_string(),
                lastname: "Doe".to_string(),
                email: "john.doe@example.com".to_string(),
                password: "hashed_password".to_string(),
                noc_transfer: "12345".to_string(),
                created_at: Some(Utc::now().naive_utc()),
                updated_at: Some(Utc::now().naive_utc()),
            }))
        });

    mock_saldo_repo
        .expect_find_by_user_id()
        .with(predicate::eq(user_id))
        .return_once(move |_| Ok(None));

    let service = SaldoService::new(Arc::new(mock_user_repo), Arc::new(mock_saldo_repo));

    let result = service.delete_saldo(user_id).await;

    assert!(result.is_err());

    let error = result.unwrap_err();

    println!("Error: {:?}", error);

    assert_eq!(error.status, "error");
    assert_eq!(
        error.message,
        format!("Saldo with id {} not found", user_id)
    );
}
