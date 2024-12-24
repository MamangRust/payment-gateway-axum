use std::sync::Arc;

use chrono::{DateTime, Utc};
use example_payment_gateway_axum::{
    abstract_trait::{
        saldo::MockSaldoRepositoryTrait,
        user::{MockUserRepositoryTrait, UserRepositoryTrait},
        withdraw::{MockWithdrawRepositoryTrait, WithdrawRepositoryTrait, WithdrawServiceTrait},
    },
    domain::request::withdraw::{CreateWithdrawRequest, UpdateWithdrawRequest},
    entities::{saldo, users, withdraws},
    services::withdraw::WithdrawService,
};
use mockall::predicate;

#[tokio::test]
async fn test_get_withdraws() {
    let mut mock_withdraw_repo = MockWithdrawRepositoryTrait::new();
    let mock_saldo_repo = MockSaldoRepositoryTrait::new();
    let mock_user_repo = MockUserRepositoryTrait::new();

    let mock_withdraws = vec![
        withdraws::Model {
            withdraw_id: 1,
            user_id: 101,
            withdraw_amount: 5000,
            withdraw_time: DateTime::<Utc>::from(Utc::now()).naive_utc(),
            created_at: Some(DateTime::<Utc>::from(Utc::now()).naive_utc()),
            updated_at: Some(DateTime::<Utc>::from(Utc::now()).naive_utc()),
        },
        withdraws::Model {
            withdraw_id: 2,
            user_id: 102,
            withdraw_amount: 7000,
            withdraw_time: DateTime::<Utc>::from(Utc::now()).naive_utc(),
            created_at: Some(DateTime::<Utc>::from(Utc::now()).naive_utc()),
            updated_at: Some(DateTime::<Utc>::from(Utc::now()).naive_utc()),
        },
    ];

    mock_withdraw_repo
        .expect_find_all()
        .return_once(move || Ok(mock_withdraws.clone()));

    let service = WithdrawService::new(
        Arc::new(mock_withdraw_repo),
        Arc::new(mock_saldo_repo),
        Arc::new(mock_user_repo),
    );

    let result = service.get_withdraws().await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.status, "success");
    assert_eq!(response.message, "Withdraw retrieved successfully");
    assert_eq!(response.data.len(), 2);
    assert_eq!(response.data[0].withdraw_id, 1);
    assert_eq!(response.data[0].user_id, 101);
    assert_eq!(response.data[1].withdraw_id, 2);
    assert_eq!(response.data[1].user_id, 102);
}

#[tokio::test]
async fn test_get_withdraw() {
    let mut mock_withdraw_repo = MockWithdrawRepositoryTrait::new();
    let mock_user_repo = MockUserRepositoryTrait::new();

    let mock_withdraw = Some(withdraws::Model {
        withdraw_id: 1,
        user_id: 101,
        withdraw_amount: 5000,
        withdraw_time: DateTime::<Utc>::from(Utc::now()).naive_utc(),
        created_at: Some(DateTime::<Utc>::from(Utc::now()).naive_utc()),
        updated_at: Some(DateTime::<Utc>::from(Utc::now()).naive_utc()),
    });

    mock_withdraw_repo
        .expect_find_by_id()
        .with(predicate::eq(1))
        .return_once(move |_| Ok(mock_withdraw.clone()));

    let service = WithdrawService::new(
        Arc::new(mock_withdraw_repo),
        Arc::new(MockSaldoRepositoryTrait::new()),
        Arc::new(mock_user_repo),
    );

    let result = service.get_withdraw(1).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.status, "success");
    assert_eq!(response.message, "Withdraw retrieved successfully");
    assert!(response.data.is_some());
    let withdraw = response.data.unwrap();
    assert_eq!(withdraw.withdraw_id, 1);
}

#[tokio::test]
async fn test_get_withdraw_not_found() {
    let mut mock_withdraw_repo = MockWithdrawRepositoryTrait::new();
    let mock_user_repo = MockUserRepositoryTrait::new();

    mock_withdraw_repo
        .expect_find_by_id()
        .with(predicate::eq(1i32))
        .return_once(move |_| Ok(None));

    let service = WithdrawService::new(
        Arc::new(mock_withdraw_repo),
        Arc::new(MockSaldoRepositoryTrait::new()),
        Arc::new(mock_user_repo),
    );

    let result = service.get_withdraw(1).await;

    assert!(result.is_err());
    let error_response = result.unwrap_err();
    assert_eq!(error_response.status, "error");
    assert_eq!(error_response.message, "Saldo with id 1 not found");
}

#[tokio::test]
async fn test_get_withdraw_users() {
    let mut mock_withdraw_repo = MockWithdrawRepositoryTrait::new();
    let mut mock_user_repo = MockUserRepositoryTrait::new();

    mock_user_repo
        .expect_find_by_id()
        .with(predicate::eq(1))
        .return_once(move |_| {
            Ok(Some(users::Model {
                user_id: 1,
                firstname: "John".to_string(),
                lastname: "Doe".to_string(),
                email: "johndoe@example.com".to_string(),
                password: "hashed_password".to_string(),
                noc_transfer: "12345".to_string(),
                created_at: Some(DateTime::<Utc>::from(Utc::now()).naive_utc()),
                updated_at: Some(DateTime::<Utc>::from(Utc::now()).naive_utc()),
            }))
        });

    let mock_withdraws = Some(vec![withdraws::Model {
        withdraw_id: 1,
        user_id: 1,
        withdraw_amount: 5000,
        withdraw_time: DateTime::<Utc>::from(Utc::now()).naive_utc(),
        created_at: Some(DateTime::<Utc>::from(Utc::now()).naive_utc()),
        updated_at: Some(DateTime::<Utc>::from(Utc::now()).naive_utc()),
    }]);

    mock_withdraw_repo
        .expect_find_by_users()
        .with(predicate::eq(1))
        .return_once(move |_| Ok(mock_withdraws.clone()));

    let service = WithdrawService::new(
        Arc::new(mock_withdraw_repo),
        Arc::new(MockSaldoRepositoryTrait::new()),
        Arc::new(mock_user_repo),
    );

    let result = service.get_withdraw_users(1).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.status, "success");
    assert_eq!(response.message, "Success");
    assert!(response.data.is_some());
    assert_eq!(response.data.unwrap().len(), 1);
}

#[tokio::test]
async fn test_get_withdraw_users_not_found() {
    let mut mock_withdraw_repo = MockWithdrawRepositoryTrait::new();
    let mock_saldo_repo = MockSaldoRepositoryTrait::new();
    let mut mock_user_repo = MockUserRepositoryTrait::new();

    mock_withdraw_repo
        .expect_find_by_users()
        .with(predicate::eq(1))
        .return_once(move |_| Ok(None));

    mock_user_repo
        .expect_find_by_id()
        .with(predicate::eq(1))
        .return_once(move |_| {
            Ok(Some(users::Model {
                user_id: 1,
                firstname: "John".to_string(),
                lastname: "Doe".to_string(),
                email: "john.doe@example.com".to_string(),
                password: "password".to_string(),
                noc_transfer: "12345".to_string(),
                created_at: None,
                updated_at: None,
            }))
        });

    let service = WithdrawService::new(
        Arc::new(mock_withdraw_repo),
        Arc::new(mock_saldo_repo),
        Arc::new(mock_user_repo),
    );

    let result = service.get_withdraw_users(1).await;

    assert!(result.is_ok());
    let response = result.unwrap();

    assert_eq!(response.status, "success");
    assert_eq!(response.message, "No withdraw found for user with id 1");
    match response.data {
        None => assert!(true),
        Some(_) => assert!(false),
    }
}

#[tokio::test]
async fn test_get_withdraw_user_success() {
    let mut mock_withdraw_repo = MockWithdrawRepositoryTrait::new();
    let mut mock_user_repo = MockUserRepositoryTrait::new();

    mock_user_repo
        .expect_find_by_id()
        .with(predicate::eq(1))
        .return_once(move |_| {
            Ok(Some(users::Model {
                user_id: 1,
                firstname: "John".to_string(),
                lastname: "Doe".to_string(),
                email: "john.doe@example.com".to_string(),
                password: "password".to_string(),
                noc_transfer: "12345".to_string(),
                created_at: None,
                updated_at: None,
            }))
        });

    mock_withdraw_repo
        .expect_find_by_user()
        .with(predicate::eq(1))
        .return_once(move |_| {
            Ok(Some(withdraws::Model {
                withdraw_id: 1,
                user_id: 1,
                withdraw_amount: 5000,
                withdraw_time: DateTime::<Utc>::from(Utc::now()).naive_utc(),
                created_at: Some(DateTime::<Utc>::from(Utc::now()).naive_utc()),
                updated_at: Some(DateTime::<Utc>::from(Utc::now()).naive_utc()),
            }))
        });

    let service = WithdrawService::new(
        Arc::new(mock_withdraw_repo),
        Arc::new(MockSaldoRepositoryTrait::new()),
        Arc::new(mock_user_repo),
    );

    let result = service.get_withdraw_user(1).await;

    assert!(result.is_ok());
    let response = result.unwrap();

    assert_eq!(response.status, "success");
    assert_eq!(response.message, "Success");

    if let Some(data) = response.data {
        assert_eq!(data.withdraw_id, 1);
        assert_eq!(data.user_id, 1);
    } else {
        panic!("Expected Some(data), found None");
    }
}

#[tokio::test]
async fn test_get_withdraw_user_not_found() {
    let mut mock_withdraw_repo = MockWithdrawRepositoryTrait::new();
    let mock_saldo_repo = MockSaldoRepositoryTrait::new();
    let mut mock_user_repo = MockUserRepositoryTrait::new();

    mock_user_repo
        .expect_find_by_id()
        .with(predicate::eq(1))
        .return_once(move |_| {
            Ok(Some(users::Model {
                user_id: 1,
                firstname: "John".to_string(),
                lastname: "Doe".to_string(),
                email: "john.doe@example.com".to_string(),
                password: "password".to_string(),
                noc_transfer: "12345".to_string(),
                created_at: None,
                updated_at: None,
            }))
        });

    mock_withdraw_repo
        .expect_find_by_user()
        .with(predicate::eq(1))
        .return_once(move |_| Ok(None));

    let service = WithdrawService::new(
        Arc::new(mock_withdraw_repo),
        Arc::new(mock_saldo_repo),
        Arc::new(mock_user_repo),
    );

    let result = service.get_withdraw_user(1).await;

    assert!(result.is_err());
    let response = result.unwrap_err();

    assert_eq!(response.status, "error");
    assert_eq!(response.message, "Topup with user id 1 not found");
}

#[tokio::test]
async fn test_get_withdraw_user_user_not_found() {
    let mut mock_withdraw_repo = MockWithdrawRepositoryTrait::new();
    let mock_saldo_repo = MockSaldoRepositoryTrait::new();
    let mut mock_user_repo = MockUserRepositoryTrait::new();

    mock_user_repo
        .expect_find_by_id()
        .with(predicate::eq(1))
        .return_once(move |_| Ok(None));

    mock_withdraw_repo
        .expect_find_by_user()
        .with(predicate::eq(1))
        .return_once(move |_| Ok(None));

    let service = WithdrawService::new(
        Arc::new(mock_withdraw_repo),
        Arc::new(mock_saldo_repo),
        Arc::new(mock_user_repo),
    );

    let result = service.get_withdraw_user(1).await;

    assert!(result.is_err());
    let response = result.unwrap_err();

    assert_eq!(response.status, "error");
    assert_eq!(response.message, "Topup with user id 1 not found");
}

#[tokio::test]
async fn test_create_withdraw_success() {
    let mut mock_withdraw_repo = MockWithdrawRepositoryTrait::new();
    let mut mock_saldo_repo = MockSaldoRepositoryTrait::new();
    let mut mock_user_repo = MockUserRepositoryTrait::new();
    let user_id = 1;
    let withdraw_amount = 100000;
    let total_balance = 1000000;

    mock_user_repo
        .expect_find_by_id()
        .with(predicate::eq(user_id))
        .return_once(move |_| {
            Ok(Some(users::Model {
                user_id,
                firstname: "John".to_string(),
                lastname: "Doe".to_string(),
                email: "john.doe@example.com".to_string(),
                password: "password".to_string(),
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
                saldo_id: 1,
                user_id,
                total_balance,
                withdraw_amount: None,
                withdraw_time: None,
                created_at: Some(Utc::now().naive_utc()),
                updated_at: Some(Utc::now().naive_utc()),
            }))
        });

    mock_saldo_repo
        .expect_update_saldo_withdraw()
        .with(predicate::always())
        .return_once(move |input| {
            Ok(saldo::Model {
                saldo_id: 1,
                user_id: input.user_id,
                total_balance: total_balance - withdraw_amount,
                withdraw_amount: Some(withdraw_amount),
                withdraw_time: Some(Utc::now().naive_utc()),
                created_at: Some(Utc::now().naive_utc()),
                updated_at: Some(Utc::now().naive_utc()),
            })
        });

    mock_withdraw_repo
        .expect_create()
        .with(predicate::always())
        .return_once(move |input| {
            Ok(withdraws::Model {
                withdraw_id: 1,
                user_id: input.user_id,
                withdraw_amount: input.withdraw_amount,
                withdraw_time: input.withdraw_time.naive_utc(),
                created_at: Some(Utc::now().naive_utc()),
                updated_at: Some(Utc::now().naive_utc()),
            })
        });

    let service = WithdrawService::new(
        Arc::new(mock_withdraw_repo),
        Arc::new(mock_saldo_repo),
        Arc::new(mock_user_repo),
    );

    let input = CreateWithdrawRequest {
        user_id,
        withdraw_amount,
        withdraw_time: Utc::now(),
    };

    let result = service.create_withdraw(&input).await;

    if let Err(ref e) = result {
        println!("Error: {:?}", e);
    }

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.status, "success");
    assert_eq!(response.message, "Withdraw created successfully");
    let data = response.data;
    assert_eq!(data.user_id, user_id);
    assert_eq!(data.withdraw_amount, withdraw_amount);
}

#[tokio::test]
async fn test_create_withdraw_insufficient_balance() {
    let mock_withdraw_repo = MockWithdrawRepositoryTrait::new();
    let mut mock_saldo_repo = MockSaldoRepositoryTrait::new();
    let mut mock_user_repo = MockUserRepositoryTrait::new();

    let user_id = 1;
    let withdraw_amount = 200000;
    let total_balance = 100000;

    mock_user_repo
        .expect_find_by_id()
        .with(predicate::eq(user_id))
        .return_once(move |_| {
            Ok(Some(users::Model {
                user_id,
                firstname: "John".to_string(),
                lastname: "Doe".to_string(),
                email: "john.doe@example.com".to_string(),
                password: "password".to_string(),
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
                saldo_id: 1,
                user_id,
                total_balance,
                withdraw_amount: None,
                withdraw_time: None,
                created_at: Some(Utc::now().naive_utc()),
                updated_at: Some(Utc::now().naive_utc()),
            }))
        });

    let service = WithdrawService::new(
        Arc::new(mock_withdraw_repo),
        Arc::new(mock_saldo_repo),
        Arc::new(mock_user_repo),
    );

    let input = CreateWithdrawRequest {
        user_id,
        withdraw_amount,
        withdraw_time: Utc::now(),
    };

    let result = service.create_withdraw(&input).await;

    if let Ok(ref response) = result {
        println!("Unexpected success: {:?}", response);
    }

    assert!(result.is_err());

    let error = result.unwrap_err();
    assert_eq!(error.status, "Error Validation");
    assert_eq!(error.message, "Insufficient balance");
}

#[tokio::test]
async fn test_create_withdraw_saldo_not_found() {
    let mock_withdraw_repo = MockWithdrawRepositoryTrait::new();
    let mut mock_saldo_repo = MockSaldoRepositoryTrait::new();
    let mut mock_user_repo = MockUserRepositoryTrait::new();

    let user_id = 1;
    let withdraw_amount = 100000;

    mock_user_repo
        .expect_find_by_id()
        .with(predicate::eq(user_id))
        .return_once(move |_| {
            Ok(Some(users::Model {
                user_id,
                firstname: "John".to_string(),
                lastname: "Doe".to_string(),
                email: "john.doe@example.com".to_string(),
                password: "password".to_string(),
                noc_transfer: "12345".to_string(),
                created_at: Some(Utc::now().naive_utc()),
                updated_at: Some(Utc::now().naive_utc()),
            }))
        });

    mock_saldo_repo
        .expect_find_by_user_id()
        .with(predicate::eq(user_id))
        .return_once(move |_| Ok(None));

    let service = WithdrawService::new(
        Arc::new(mock_withdraw_repo),
        Arc::new(mock_saldo_repo),
        Arc::new(mock_user_repo),
    );

    let input = CreateWithdrawRequest {
        user_id,
        withdraw_amount,
        withdraw_time: Utc::now(),
    };

    let result = service.create_withdraw(&input).await;

    if let Ok(ref response) = result {
        println!("Unexpected success: {:?}", response);
    }

    assert!(result.is_err());

    let error = result.unwrap_err();
    assert_eq!(error.status, "error");
    assert_eq!(error.message, format!("Saldo not found"));
}

#[tokio::test]
async fn test_create_withdraw_validation_error() {
    let mock_withdraw_repo = MockWithdrawRepositoryTrait::new();
    let mock_saldo_repo = MockSaldoRepositoryTrait::new();
    let mock_user_repo = MockUserRepositoryTrait::new();

    let user_id = 1;
    let withdraw_amount = -100;

    let service = WithdrawService::new(
        Arc::new(mock_withdraw_repo),
        Arc::new(mock_saldo_repo),
        Arc::new(mock_user_repo),
    );

    let input = CreateWithdrawRequest {
        user_id,
        withdraw_amount,
        withdraw_time: Utc::now(),
    };

    let result = service.create_withdraw(&input).await;

    if let Ok(ref response) = result {
        println!("Unexpected success: {:?}", response);
    }

    assert!(result.is_err());

    let error = result.unwrap_err();
    assert_eq!(error.status, "Error Validation");
    assert_eq!(error.message, "Withdraw amount must be at least 50,000");
}

#[tokio::test]
async fn test_update_withdraw_success() {
    let mut mock_withdraw_repo = MockWithdrawRepositoryTrait::new();
    let mut mock_saldo_repo = MockSaldoRepositoryTrait::new();
    let mock_user_repo = MockUserRepositoryTrait::new();

    let user_id = 1;
    let withdraw_id = 1;
    let withdraw_amount = 500000;
    let total_balance = 10000000;

    mock_withdraw_repo
        .expect_find_by_id()
        .with(predicate::eq(withdraw_id))
        .return_once(move |_| {
            Ok(Some(withdraws::Model {
                withdraw_id,
                user_id,
                withdraw_amount: 3000,
                withdraw_time: Utc::now().naive_utc(),
                created_at: Some(Utc::now().naive_utc()),
                updated_at: Some(Utc::now().naive_utc()),
            }))
        });

    mock_saldo_repo
        .expect_find_by_user_id()
        .with(predicate::eq(user_id))
        .return_once(move |_| {
            Ok(Some(saldo::Model {
                saldo_id: 1,
                user_id,
                total_balance,
                withdraw_amount: Some(3000),
                withdraw_time: Some(Utc::now().naive_utc()),
                created_at: Some(Utc::now().naive_utc()),
                updated_at: Some(Utc::now().naive_utc()),
            }))
        });

    mock_withdraw_repo
        .expect_update()
        .with(predicate::always())
        .return_once(move |input| {
            Ok(withdraws::Model {
                withdraw_id,
                user_id: input.user_id,
                withdraw_amount: input.withdraw_amount,
                withdraw_time: input.withdraw_time.naive_utc(),
                created_at: Some(Utc::now().naive_utc()),
                updated_at: Some(Utc::now().naive_utc()),
            })
        });

    mock_saldo_repo
        .expect_update_saldo_withdraw()
        .with(predicate::always())
        .return_once(move |input| {
            Ok(saldo::Model {
                saldo_id: 1,
                user_id: input.user_id,
                total_balance: total_balance - withdraw_amount,
                withdraw_amount: Some(input.withdraw_amount.unwrap_or(0)),
                withdraw_time: Some(Utc::now().naive_utc()),
                created_at: Some(Utc::now().naive_utc()),
                updated_at: Some(Utc::now().naive_utc()),
            })
        });

    let service = WithdrawService::new(
        Arc::new(mock_withdraw_repo),
        Arc::new(mock_saldo_repo),
        Arc::new(mock_user_repo),
    );

    let input = UpdateWithdrawRequest {
        withdraw_id,
        user_id,
        withdraw_amount,
        withdraw_time: Utc::now(),
    };

    let result = service.update_withdraw(&input).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.status, "success");
    assert_eq!(response.message, "Withdraw updated successfully");
    let data = response.data.unwrap();
    assert_eq!(data.user_id, user_id);
    assert_eq!(data.withdraw_amount, withdraw_amount);
}

#[tokio::test]
async fn test_update_withdraw_validation_error() {
    let mock_withdraw_repo = MockWithdrawRepositoryTrait::new();
    let mock_saldo_repo = MockSaldoRepositoryTrait::new();
    let mock_user_repo = MockUserRepositoryTrait::new();

    let user_id = 1;
    let withdraw_id = 1;
    let withdraw_amount = -5000;

    let service = WithdrawService::new(
        Arc::new(mock_withdraw_repo),
        Arc::new(mock_saldo_repo),
        Arc::new(mock_user_repo),
    );

    let input = UpdateWithdrawRequest {
        withdraw_id,
        user_id,
        withdraw_amount,
        withdraw_time: Utc::now(),
    };

    let result = service.update_withdraw(&input).await;

    assert!(result.is_err());
    let error = result.unwrap_err();
    assert_eq!(error.status, "Error Validation");
    assert_eq!(error.message, "Withdraw amount must be at least 50,000");
}

#[tokio::test]
async fn test_update_withdraw_saldo_not_found() {
    let mut mock_withdraw_repo = MockWithdrawRepositoryTrait::new();
    let mut mock_saldo_repo = MockSaldoRepositoryTrait::new();
    let mut mock_user_repo = MockUserRepositoryTrait::new();

    let user_id = 1;
    let withdraw_id = 1;
    let withdraw_amount = 100000;

    mock_user_repo
        .expect_find_by_id()
        .with(predicate::eq(user_id))
        .return_once(move |_| {
            Ok(Some(users::Model {
                user_id,
                firstname: "John".to_string(),
                lastname: "Doe".to_string(),
                email: "john.doe@example.com".to_string(),
                password: "password".to_string(),
                noc_transfer: "12345".to_string(),
                created_at: Some(Utc::now().naive_utc()),
                updated_at: Some(Utc::now().naive_utc()),
            }))
        });

    mock_withdraw_repo
        .expect_find_by_id()
        .with(predicate::eq(withdraw_id))
        .return_once(move |_| {
            Ok(Some(withdraws::Model {
                withdraw_id,
                user_id,
                withdraw_amount: 50000,
                withdraw_time: Utc::now().naive_utc(),
                created_at: Some(Utc::now().naive_utc()),
                updated_at: Some(Utc::now().naive_utc()),
            }))
        });

    mock_saldo_repo
        .expect_find_by_user_id()
        .with(predicate::eq(user_id))
        .return_once(move |_| Ok(None));

    let service = WithdrawService::new(
        Arc::new(mock_withdraw_repo),
        Arc::new(mock_saldo_repo),
        Arc::new(mock_user_repo),
    );

    let input = UpdateWithdrawRequest {
        withdraw_id,
        user_id,
        withdraw_amount,
        withdraw_time: Utc::now(),
    };

    let result = service.update_withdraw(&input).await;

    if let Ok(ref response) = result {
        println!("Unexpected success: {:?}", response);
    }

    assert!(result.is_err());

    let error = result.unwrap_err();
    assert_eq!(error.status, "error");
    assert_eq!(error.message, format!("Saldo not found"));
}
