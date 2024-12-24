use std::sync::Arc;

use chrono::{DateTime, Utc};
use example_payment_gateway_axum::{
    abstract_trait::{
        saldo::{MockSaldoRepositoryTrait, SaldoServiceTrait},
        topup::{MockTopupRepositoryTrait, TopupRepositoryTrait, TopupServiceTrait},
        user::{MockUserRepositoryTrait, UserRepositoryTrait},
    },
    domain::request::topup::{CreateTopupRequest, UpdateTopupRequest},
    entities::{saldo, topups, users},
    services::{saldo::SaldoService, topup::TopupService},
};
use mockall::predicate;
use sea_orm::DbErr;

#[tokio::test]
async fn test_get_topups() {
    let mut mock_topup_repo = MockTopupRepositoryTrait::new();
    let mut _mock_saldo_repo = MockSaldoRepositoryTrait::new();
    let mock_user_repo = MockUserRepositoryTrait::new();

    let mock_topups = vec![
        topups::Model {
            topup_id: 1,
            user_id: 1,
            topup_no: "TOP12345".to_string(),
            topup_amount: 1000,
            topup_method: "Bank Transfer".to_string(),
            topup_time: DateTime::from_timestamp(1_634_944_800, 0)
                .unwrap()
                .naive_utc(),
            created_at: Some(DateTime::<Utc>::from(Utc::now()).naive_utc()),
            updated_at: Some(DateTime::<Utc>::from(Utc::now()).naive_utc()),
        },
        topups::Model {
            topup_id: 2,
            user_id: 1,
            topup_no: "TOP12345".to_string(),
            topup_amount: 1000,
            topup_method: "Bank Transfer".to_string(),
            topup_time: DateTime::from_timestamp(1_634_944_800, 0)
                .unwrap()
                .naive_utc(),
            created_at: Some(DateTime::<Utc>::from(Utc::now()).naive_utc()),
            updated_at: Some(DateTime::<Utc>::from(Utc::now()).naive_utc()),
        },
    ];

    mock_topup_repo
        .expect_find_all()
        .return_once(move || Ok(mock_topups.clone()));

    let service = TopupService::new(
        Arc::new(mock_topup_repo),
        Arc::new(_mock_saldo_repo),
        Arc::new(mock_user_repo),
    );

    let result = service.get_topups().await;

    let response = result.unwrap();

    assert_eq!(response.status, "success");
    assert_eq!(response.message, "Topup retrieved successfully");

    let data = response.data;

    assert_eq!(data.len(), 2);
    assert_eq!(data[0].topup_id, 1);
    assert_eq!(data[0].user_id, 1);
    assert_eq!(data[0].topup_no, "TOP12345");
    assert_eq!(data[0].topup_amount, 1000);
    assert_eq!(data[0].topup_method, "Bank Transfer");
}

#[tokio::test]
async fn test_get_topups_empty_data() {
    let mut mock_topup_repo = MockTopupRepositoryTrait::new();
    let mut _mock_saldo_repo = MockSaldoRepositoryTrait::new();
    let mock_user_repo = MockUserRepositoryTrait::new();

    mock_topup_repo
        .expect_find_all()
        .return_once(move || Ok(Vec::new()));

    let service = TopupService::new(
        Arc::new(mock_topup_repo),
        Arc::new(_mock_saldo_repo),
        Arc::new(mock_user_repo),
    );

    let result = service.get_topups().await;

    let response = result.unwrap();

    assert_eq!(response.status, "success");
    assert_eq!(response.message, "Topup retrieved successfully");

    let data = response.data;

    assert!(data.is_empty());
}

#[tokio::test]
async fn test_get_topup_success() {
    let mut mock_topup_repo = MockTopupRepositoryTrait::new();
    let mut _mock_saldo_repo = MockSaldoRepositoryTrait::new();
    let mock_user_repo = MockUserRepositoryTrait::new();

    let mock_topup = topups::Model {
        topup_id: 1,
        user_id: 1,
        topup_no: "TOP12345".to_string(),
        topup_amount: 1000,
        topup_method: "Bank Transfer".to_string(),
        topup_time: DateTime::from_timestamp(1_634_944_800, 0)
            .unwrap()
            .naive_utc(),
        created_at: Some(DateTime::<Utc>::from(Utc::now()).naive_utc()),
        updated_at: Some(DateTime::<Utc>::from(Utc::now()).naive_utc()),
    };

    mock_topup_repo
        .expect_find_by_id()
        .with(predicate::eq(1))
        .return_once(move |_| Ok(Some(mock_topup.clone())));

    let service = TopupService::new(
        Arc::new(mock_topup_repo),
        Arc::new(_mock_saldo_repo),
        Arc::new(mock_user_repo),
    );

    let result = service.get_topup(1).await;

    let response = result.unwrap();

    assert_eq!(response.status, "success");
    assert_eq!(response.message, "Topup retrieved successfully");

    let data = response.data.unwrap();

    assert_eq!(data.topup_id, 1);
    assert_eq!(data.user_id, 1);
    assert_eq!(data.topup_no, "TOP12345");
    assert_eq!(data.topup_amount, 1000);
    assert_eq!(data.topup_method, "Bank Transfer");
}

#[tokio::test]
async fn test_get_topup_not_found() {
    let mut mock_topup_repo = MockTopupRepositoryTrait::new();
    let mut _mock_saldo_repo = MockSaldoRepositoryTrait::new();
    let mock_user_repo = MockUserRepositoryTrait::new();

    mock_topup_repo
        .expect_find_by_id()
        .with(predicate::eq(1))
        .return_once(move |_| Ok(None));

    let service = TopupService::new(
        Arc::new(mock_topup_repo),
        Arc::new(_mock_saldo_repo),
        Arc::new(mock_user_repo),
    );

    let result = service.get_topup(1).await;

    let response = result.unwrap_err();

    assert_eq!(response.status, "error");
    assert_eq!(response.message, "Topup with id 1 not found");
}

#[tokio::test]
async fn test_get_topup_error() {
    let mut mock_topup_repo = MockTopupRepositoryTrait::new();
    let mut _mock_saldo_repo = MockSaldoRepositoryTrait::new();
    let mock_user_repo = MockUserRepositoryTrait::new();

    mock_topup_repo
        .expect_find_by_id()
        .with(predicate::eq(1))
        .return_once(move |_| Err(DbErr::RecordNotFound("Topup not found".to_owned())));

    let service = TopupService::new(
        Arc::new(mock_topup_repo),
        Arc::new(_mock_saldo_repo),
        Arc::new(mock_user_repo),
    );

    let result = service.get_topup(1).await;

    let response = result.unwrap_err();

    assert_eq!(response.status, "error");
    assert_eq!(response.message, "Database error occurred");
}

#[tokio::test]
async fn test_get_topup_users() {
    let mut mock_topup_repo = MockTopupRepositoryTrait::new();
    let mut _mock_saldo_repo = MockSaldoRepositoryTrait::new();
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

    let mock_topups = vec![
        Some(topups::Model {
            topup_id: 1,
            user_id: 1,
            topup_no: "TOP12345".to_string(),
            topup_amount: 1000,
            topup_method: "Bank Transfer".to_string(),
            topup_time: DateTime::from_timestamp(1_634_944_800, 0)
                .unwrap()
                .naive_utc(),
            created_at: Some(DateTime::<Utc>::from(Utc::now()).naive_utc()),
            updated_at: Some(DateTime::<Utc>::from(Utc::now()).naive_utc()),
        }),
        Some(topups::Model {
            topup_id: 2,
            user_id: 1,
            topup_no: "TOP12345".to_string(),
            topup_amount: 1000,
            topup_method: "Bank Transfer".to_string(),
            topup_time: DateTime::from_timestamp(1_634_944_800, 0)
                .unwrap()
                .naive_utc(),
            created_at: Some(DateTime::<Utc>::from(Utc::now()).naive_utc()),
            updated_at: Some(DateTime::<Utc>::from(Utc::now()).naive_utc()),
        }),
    ];

    mock_topup_repo
        .expect_find_by_users()
        .with(predicate::eq(user_id))
        .return_once(move |_| Ok(mock_topups.clone()));

    let service = TopupService::new(
        Arc::new(mock_topup_repo),
        Arc::new(_mock_saldo_repo),
        Arc::new(mock_user_repo),
    );

    let result = service.get_topup_users(user_id).await;

    let response = result.unwrap();

    assert_eq!(response.status, "success");
    assert_eq!(response.message, "Success");
}

#[tokio::test]
async fn test_get_topup_users_error() {
    let mut mock_topup_repo = MockTopupRepositoryTrait::new();
    let mut _mock_saldo_repo = MockSaldoRepositoryTrait::new();
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
                password: "hashed_password".to_string(),
                noc_transfer: "12345".to_string(),
                created_at: Some(Utc::now().naive_utc()),
                updated_at: Some(Utc::now().naive_utc()),
            }))
        });

    mock_topup_repo
        .expect_find_by_users()
        .with(predicate::eq(1))
        .return_once(move |_| Err(DbErr::RecordNotFound("Topup not found".to_owned())));

    let service = TopupService::new(
        Arc::new(mock_topup_repo),
        Arc::new(_mock_saldo_repo),
        Arc::new(mock_user_repo),
    );

    let result = service.get_topup_users(1).await;

    let response = result.unwrap_err();

    assert_eq!(response.status, "error");
    assert_eq!(response.message, "Database error occurred");
}

#[tokio::test]
async fn test_get_topup_users_not_found() {
    let mut mock_topup_repo = MockTopupRepositoryTrait::new();
    let mut _mock_saldo_repo = MockSaldoRepositoryTrait::new();
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
                password: "hashed_password".to_string(),
                noc_transfer: "12345".to_string(),
                created_at: Some(Utc::now().naive_utc()),
                updated_at: Some(Utc::now().naive_utc()),
            }))
        });

    mock_topup_repo
        .expect_find_by_users()
        .with(predicate::eq(1))
        .return_once(move |_| Ok(vec![]));

    let service = TopupService::new(
        Arc::new(mock_topup_repo),
        Arc::new(_mock_saldo_repo),
        Arc::new(mock_user_repo),
    );

    let result = service.get_topup_users(1).await;

    let response = result.unwrap();

    assert_eq!(response.status, "success");
    assert_eq!(response.message, "No topup found for user with id 1");
}

#[tokio::test]
async fn test_get_topup_user_success() {
    let mut mock_topup_repo = MockTopupRepositoryTrait::new();
    let mut _mock_saldo_repo = MockSaldoRepositoryTrait::new();
    let mut mock_user_repo = MockUserRepositoryTrait::new();

    let user_id = 1;

    mock_user_repo
        .expect_find_by_id()
        .with(predicate::eq(user_id))
        .return_once(move |_| {
            Ok(Some(users::Model {
                user_id: 1,
                firstname: "John".to_string(),
                lastname: "Doe".to_string(),
                email: "john.doe@example.com".to_string(),
                password: "hashed_password".to_string(),
                noc_transfer: "12345".to_string(),
                created_at: Some(Utc::now().naive_utc()),
                updated_at: Some(Utc::now().naive_utc()),
            }))
        });

    mock_topup_repo
        .expect_find_by_user()
        .with(predicate::eq(user_id))
        .return_once(move |_| {
            Ok(Some(topups::Model {
                topup_id: 1,
                user_id: 1,
                topup_no: "TOP12345".to_string(),
                topup_amount: 1000,
                topup_method: "Bank Transfer".to_string(),
                topup_time: DateTime::from_timestamp(1_634_944_800, 0)
                    .unwrap()
                    .naive_utc(),
                created_at: Some(DateTime::<Utc>::from(Utc::now()).naive_utc()),
                updated_at: Some(DateTime::<Utc>::from(Utc::now()).naive_utc()),
            }))
        });

    let service = TopupService::new(
        Arc::new(mock_topup_repo),
        Arc::new(_mock_saldo_repo),
        Arc::new(mock_user_repo),
    );

    let result = service.get_topup_user(1).await;

    let response = result.unwrap();

    assert_eq!(response.status, "success");
    assert_eq!(response.message, "Success");

    let data = response.data.unwrap();

    assert_eq!(data.topup_id, 1);
    assert_eq!(data.user_id, 1);
    assert_eq!(data.topup_no, "TOP12345");
    assert_eq!(data.topup_amount, 1000);
    assert_eq!(data.topup_method, "Bank Transfer");
}

#[tokio::test]
async fn test_get_topup_user_not_found() {
    let mut mock_topup_repo = MockTopupRepositoryTrait::new();
    let mut _mock_saldo_repo = MockSaldoRepositoryTrait::new();
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
                password: "hashed_password".to_string(),
                noc_transfer: "12345".to_string(),
                created_at: Some(Utc::now().naive_utc()),
                updated_at: Some(Utc::now().naive_utc()),
            }))
        });

    mock_topup_repo
        .expect_find_by_user()
        .with(predicate::eq(1))
        .return_once(move |_| Ok(None));

    let service = TopupService::new(
        Arc::new(mock_topup_repo),
        Arc::new(_mock_saldo_repo),
        Arc::new(mock_user_repo),
    );

    let result = service.get_topup_user(1).await;

    let error = result.unwrap_err();

    println!("Error: {:?}", error);

    assert_eq!(error.status, "error");
    assert_eq!(error.message, "Topup with user id 1 not found");
}

#[tokio::test]
async fn test_get_topup_user_error() {
    let mut mock_topup_repo = MockTopupRepositoryTrait::new();
    let mut _mock_saldo_repo = MockSaldoRepositoryTrait::new();
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
                password: "hashed_password".to_string(),
                noc_transfer: "12345".to_string(),
                created_at: Some(Utc::now().naive_utc()),
                updated_at: Some(Utc::now().naive_utc()),
            }))
        });

    mock_topup_repo
        .expect_find_by_user()
        .with(predicate::eq(1))
        .return_once(move |_| Err(DbErr::RecordNotFound("Topup not found".to_owned())));

    let service = TopupService::new(
        Arc::new(mock_topup_repo),
        Arc::new(_mock_saldo_repo),
        Arc::new(mock_user_repo),
    );

    let result = service.get_topup_user(1).await;

    let error = result.unwrap_err();

    println!("Error: {:?}", error);

    assert_eq!(error.status, "error");
    assert_eq!(error.message, "Database error occurred");
}

#[tokio::test]
async fn test_create_topup_success() {
    let mut mock_user_repo = MockUserRepositoryTrait::new();
    let mut mock_topup_repo = MockTopupRepositoryTrait::new();
    let mut mock_saldo_repo = MockSaldoRepositoryTrait::new();

    mock_user_repo.expect_find_by_id().return_once(|_| {
        Ok(Some(users::Model {
            user_id: 1,
            firstname: "John".to_string(),
            lastname: "Doe".to_string(),
            email: "john.doe@example.com".to_string(),
            password: "hashed_password".to_string(),
            noc_transfer: "12345".to_string(),
            created_at: Some(Utc::now().naive_utc()),
            updated_at: Some(Utc::now().naive_utc()),
        }))
    });

    mock_topup_repo.expect_create().return_once(|_| {
        Ok(topups::Model {
            topup_id: 1,
            topup_no: "TOP12345".to_string(),
            user_id: 1,
            topup_amount: 100000,
            topup_method: "mandiri".to_string(),
            topup_time: DateTime::from_timestamp(1_634_944_800, 0)
                .unwrap()
                .naive_utc(),
            created_at: Some(Utc::now().naive_utc()),
            updated_at: Some(Utc::now().naive_utc()),
        })
    });

    mock_saldo_repo.expect_find_by_user_id().return_once(|_| {
        Ok(Some(saldo::Model {
            saldo_id: 1,
            user_id: 1,
            total_balance: 100000,
            withdraw_amount: None,
            withdraw_time: None,
            created_at: Some(Utc::now().naive_utc()),
            updated_at: Some(Utc::now().naive_utc()),
        }))
    });
    mock_saldo_repo.expect_update_balance().return_once(|_| {
        Ok(saldo::Model {
            saldo_id: 1,
            user_id: 1,
            total_balance: 200000,
            withdraw_amount: None,
            withdraw_time: None,
            created_at: Some(Utc::now().naive_utc()),
            updated_at: Some(Utc::now().naive_utc()),
        })
    });

    let service = TopupService::new(
        Arc::new(mock_topup_repo),
        Arc::new(mock_saldo_repo),
        Arc::new(mock_user_repo),
    );
    let input = CreateTopupRequest {
        user_id: 1,
        topup_no: "TOP12345".to_string(),
        topup_amount: 100000,
        topup_method: "mandiri".to_string(),
    };

    let result = service.create_topup(&input).await;

    let response = result.unwrap();
    assert_eq!(response.status, "success");
    assert_eq!(response.message, "Topup created successfully");
    assert_eq!(response.data.topup_amount, 100000);
}

#[tokio::test]
async fn test_create_topup_validation_error() {
    let mut mock_user_repo = MockUserRepositoryTrait::new();
    let mut mock_topup_repo = MockTopupRepositoryTrait::new();
    let mut mock_saldo_repo = MockSaldoRepositoryTrait::new();

    mock_user_repo.expect_find_by_id().return_once(|_| {
        Ok(Some(users::Model {
            user_id: 1,
            firstname: "John".to_string(),
            lastname: "Doe".to_string(),
            email: "john.doe@example.com".to_string(),
            password: "hashed_password".to_string(),
            noc_transfer: "12345".to_string(),
            created_at: Some(Utc::now().naive_utc()),
            updated_at: Some(Utc::now().naive_utc()),
        }))
    });

    mock_topup_repo.expect_create().return_once(|_| {
        Ok(topups::Model {
            topup_id: 1,
            topup_no: "TOP12345".to_string(),
            user_id: 1,
            topup_amount: 100000,
            topup_method: "mandiri".to_string(),
            topup_time: DateTime::from_timestamp(1_634_944_800, 0)
                .unwrap()
                .naive_utc(),
            created_at: Some(Utc::now().naive_utc()),
            updated_at: Some(Utc::now().naive_utc()),
        })
    });

    mock_saldo_repo.expect_find_by_user_id().return_once(|_| {
        Ok(Some(saldo::Model {
            saldo_id: 1,
            user_id: 1,
            total_balance: 100000,
            withdraw_amount: None,
            withdraw_time: None,
            created_at: Some(Utc::now().naive_utc()),
            updated_at: Some(Utc::now().naive_utc()),
        }))
    });
    mock_saldo_repo.expect_update_balance().return_once(|_| {
        Ok(saldo::Model {
            saldo_id: 1,
            user_id: 1,
            total_balance: 200000,
            withdraw_amount: None,
            withdraw_time: None,
            created_at: Some(Utc::now().naive_utc()),
            updated_at: Some(Utc::now().naive_utc()),
        })
    });

    let service = TopupService::new(
        Arc::new(mock_topup_repo),
        Arc::new(mock_saldo_repo),
        Arc::new(mock_user_repo),
    );
    let input = CreateTopupRequest {
        user_id: 1,
        topup_no: "TOP12345".to_string(),
        topup_amount: 5000,
        topup_method: "mandiri".to_string(),
    };

    let result = service.create_topup(&input).await;

    let error = result.unwrap_err();

    assert_eq!(error.status, "Error Validation");
    assert!(error
        .message
        .contains("Topup amount must be greater than or equal to 50000"));
}

#[tokio::test]
async fn test_update_topup_success() {
    let mut mock_user_repo = MockUserRepositoryTrait::new();
    let mut mock_topup_repo = MockTopupRepositoryTrait::new();
    let mut mock_saldo_repo = MockSaldoRepositoryTrait::new();

    mock_user_repo.expect_find_by_id().return_once(|_| {
        Ok(Some(users::Model {
            user_id: 1,
            firstname: "John".to_string(),
            lastname: "Doe".to_string(),
            email: "john.doe@example.com".to_string(),
            password: "hashed_password".to_string(),
            noc_transfer: "12345".to_string(),
            created_at: Some(Utc::now().naive_utc()),
            updated_at: Some(Utc::now().naive_utc()),
        }))
    });

    mock_topup_repo
        .expect_find_by_id()
        .with(mockall::predicate::eq(1))
        .returning(|_| {
            Ok(Some(topups::Model {
                topup_id: 1,
                topup_no: "TOP12345".to_string(),
                user_id: 1,
                topup_amount: 150000,
                topup_method: "mandiri".to_string(),
                topup_time: DateTime::from_timestamp(1_634_944_800, 0)
                    .unwrap()
                    .naive_utc(),
                created_at: Some(Utc::now().naive_utc()),
                updated_at: Some(Utc::now().naive_utc()),
            }))
        });

    mock_topup_repo
        .expect_find_by_id()
        .with(mockall::predicate::eq(1))
        .return_once(|_| {
            Ok(Some(topups::Model {
                topup_id: 1,
                topup_no: "TOP12345".to_string(),
                user_id: 1,
                topup_amount: 150000,
                topup_method: "mandiri".to_string(),
                topup_time: DateTime::from_timestamp(1_634_944_800, 0)
                    .unwrap()
                    .naive_utc(),
                created_at: Some(Utc::now().naive_utc()),
                updated_at: Some(Utc::now().naive_utc()),
            }))
        });

    // Mock update topup amount
    mock_topup_repo.expect_update_amount().return_once(|_| {
        Ok(topups::Model {
            topup_id: 1,
            topup_no: "TOP12345".to_string(),
            user_id: 1,
            topup_amount: 150000,
            topup_method: "mandiri".to_string(),
            topup_time: DateTime::from_timestamp(1_634_944_800, 0)
                .unwrap()
                .naive_utc(),
            created_at: Some(Utc::now().naive_utc()),
            updated_at: Some(Utc::now().naive_utc()),
        })
    });

    mock_saldo_repo.expect_find_by_user_id().return_once(|_| {
        Ok(Some(saldo::Model {
            saldo_id: 1,
            user_id: 1,
            total_balance: 100000,
            withdraw_amount: None,
            withdraw_time: None,
            created_at: Some(Utc::now().naive_utc()),
            updated_at: Some(Utc::now().naive_utc()),
        }))
    });

    mock_saldo_repo.expect_update_balance().return_once(|_| {
        Ok(saldo::Model {
            saldo_id: 1,
            user_id: 1,
            total_balance: 150000,
            withdraw_amount: None,
            withdraw_time: None,
            created_at: Some(Utc::now().naive_utc()),
            updated_at: Some(Utc::now().naive_utc()),
        })
    });

    let service = TopupService::new(
        Arc::new(mock_topup_repo),
        Arc::new(mock_saldo_repo),
        Arc::new(mock_user_repo),
    );

    let input = UpdateTopupRequest {
        user_id: 1,
        topup_id: 1,
        topup_amount: 150000,
        topup_method: "mandiri".to_string(),
    };

    let result = service.update_topup(&input).await;

    let response = result.unwrap();
    assert_eq!(response.status, "success");
    assert_eq!(response.message, "Topup updated successfully");
    assert!(response.data.is_some());
    assert_eq!(response.data.unwrap().topup_amount, 150000);
}

#[tokio::test]
async fn test_update_topup_validation_error() {
    let mut mock_user_repo = MockUserRepositoryTrait::new();
    let mut mock_topup_repo = MockTopupRepositoryTrait::new();
    let mut mock_saldo_repo = MockSaldoRepositoryTrait::new();

    mock_user_repo.expect_find_by_id().return_once(|_| {
        Ok(Some(users::Model {
            user_id: 1,
            firstname: "John".to_string(),
            lastname: "Doe".to_string(),
            email: "john.doe@example.com".to_string(),
            password: "hashed_password".to_string(),
            noc_transfer: "12345".to_string(),
            created_at: Some(Utc::now().naive_utc()),
            updated_at: Some(Utc::now().naive_utc()),
        }))
    });

    mock_topup_repo
        .expect_find_by_id()
        .with(mockall::predicate::eq(1))
        .returning(|_| {
            Ok(Some(topups::Model {
                topup_id: 1,
                topup_no: "TOP12345".to_string(),
                user_id: 1,
                topup_amount: 150000,
                topup_method: "mandiri".to_string(),
                topup_time: DateTime::from_timestamp(1_634_944_800, 0)
                    .unwrap()
                    .naive_utc(),
                created_at: Some(Utc::now().naive_utc()),
                updated_at: Some(Utc::now().naive_utc()),
            }))
        });

    mock_topup_repo
        .expect_find_by_id()
        .with(mockall::predicate::eq(1))
        .return_once(|_| {
            Ok(Some(topups::Model {
                topup_id: 1,
                topup_no: "TOP12345".to_string(),
                user_id: 1,
                topup_amount: 150000,
                topup_method: "mandiri".to_string(),
                topup_time: DateTime::from_timestamp(1_634_944_800, 0)
                    .unwrap()
                    .naive_utc(),
                created_at: Some(Utc::now().naive_utc()),
                updated_at: Some(Utc::now().naive_utc()),
            }))
        });

    mock_topup_repo.expect_update_amount().return_once(|_| {
        Ok(topups::Model {
            topup_id: 1,
            topup_no: "TOP12345".to_string(),
            user_id: 1,
            topup_amount: 150000,
            topup_method: "mandiri".to_string(),
            topup_time: DateTime::from_timestamp(1_634_944_800, 0)
                .unwrap()
                .naive_utc(),
            created_at: Some(Utc::now().naive_utc()),
            updated_at: Some(Utc::now().naive_utc()),
        })
    });

    mock_saldo_repo.expect_find_by_user_id().return_once(|_| {
        Ok(Some(saldo::Model {
            saldo_id: 1,
            user_id: 1,
            total_balance: 100000,
            withdraw_amount: None,
            withdraw_time: None,
            created_at: Some(Utc::now().naive_utc()),
            updated_at: Some(Utc::now().naive_utc()),
        }))
    });

    mock_saldo_repo.expect_update_balance().return_once(|_| {
        Ok(saldo::Model {
            saldo_id: 1,
            user_id: 1,
            total_balance: 150000,
            withdraw_amount: None,
            withdraw_time: None,
            created_at: Some(Utc::now().naive_utc()),
            updated_at: Some(Utc::now().naive_utc()),
        })
    });

    let service = TopupService::new(
        Arc::new(mock_topup_repo),
        Arc::new(mock_saldo_repo),
        Arc::new(mock_user_repo),
    );

    let input = UpdateTopupRequest {
        user_id: 1,
        topup_id: 1,
        topup_amount: 50000,
        topup_method: "mandiri".to_string(),
    };

    let result = service.update_topup(&input).await;

    let response = result.unwrap_err();

    assert_eq!(response.status, "Error Validation");
    assert!(response
        .message
        .contains("Topup amount must be greater than or equal to 50000"));
}

#[tokio::test]
async fn test_delete_topup_success() {
    let mut mock_user_repo = MockUserRepositoryTrait::new();
    let mut _mock_saldo_repo = MockSaldoRepositoryTrait::new();
    let mut mock_topup_repo = MockTopupRepositoryTrait::new();

    mock_user_repo.expect_find_by_id().return_once(|id| {
        assert_eq!(id, 1);
        Ok(Some(users::Model {
            user_id: id,
            firstname: "John".to_string(),
            lastname: "Doe".to_string(),
            email: "john.doe@example.com".to_string(),
            password: "hashed_password".to_string(),
            noc_transfer: "12345".to_string(),
            created_at: Some(Utc::now().naive_utc()),
            updated_at: Some(Utc::now().naive_utc()),
        }))
    });

    mock_topup_repo
        .expect_find_by_user()
        .return_once(|user_id| {
            assert_eq!(user_id, 1);
            Ok(Some(topups::Model {
                topup_id: 1,
                topup_no: "TOP12345".to_string(),
                user_id,
                topup_amount: 50000,
                topup_method: "mandiri".to_string(),
                topup_time: Utc::now().naive_utc(),
                created_at: Some(Utc::now().naive_utc()),
                updated_at: Some(Utc::now().naive_utc()),
            }))
        });

    mock_topup_repo.expect_delete().return_once(|topup_id| {
        assert_eq!(topup_id, 1);
        Ok(())
    });

    let service = TopupService::new(
        Arc::new(mock_topup_repo),
        Arc::new(_mock_saldo_repo),
        Arc::new(mock_user_repo),
    );

    let result = service.delete_topup(1).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.status, "success");
    assert_eq!(response.message, "Topup deleted successfully");
}
