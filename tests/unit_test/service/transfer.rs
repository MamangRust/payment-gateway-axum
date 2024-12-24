use std::sync::Arc;

use chrono::{DateTime, Utc};
use example_payment_gateway_axum::{
    abstract_trait::{
        saldo::{MockSaldoRepositoryTrait, SaldoRepositoryTrait},
        transfer::{MockTransferRepositoryTrait, TransferRepositoryTrait, TransferServiceTrait},
        user::{MockUserRepositoryTrait, UserRepositoryTrait},
    },
    domain::request::transfer::{CreateTransferRequest, UpdateTransferRequest},
    entities::{saldo, transfers, users},
    services::transfer::TransferService,
};
use mockall::predicate;

#[tokio::test]
async fn test_get_transfers() {
    let mut mock_transfer_repo = MockTransferRepositoryTrait::new();
    let mut mock_saldo_repo = MockSaldoRepositoryTrait::new();
    let mut mock_user_repo = MockUserRepositoryTrait::new();

    let mock_transfers = vec![
        transfers::Model {
            transfer_id: 1,
            transfer_from: 1,
            transfer_to: 2,
            transfer_amount: 10000,
            transfer_time: DateTime::<Utc>::from(Utc::now()).naive_utc(),
            created_at: Some(Utc::now().naive_utc()),
            updated_at: Some(Utc::now().naive_utc()),
        },
        transfers::Model {
            transfer_id: 2,
            transfer_from: 2,
            transfer_to: 1,
            transfer_amount: 5000,
            transfer_time: DateTime::<Utc>::from(Utc::now()).naive_utc(),
            created_at: Some(Utc::now().naive_utc()),
            updated_at: Some(Utc::now().naive_utc()),
        },
    ];

    mock_transfer_repo
        .expect_find_all()
        .return_once(move || Ok(mock_transfers.clone()));

    let transfer_service = TransferService::new(
        Arc::new(mock_transfer_repo),
        Arc::new(mock_saldo_repo),
        Arc::new(mock_user_repo),
    );

    let result = transfer_service.get_transfers().await;

    assert!(result.is_ok());

    let response = result.unwrap();

    assert_eq!(response.status, "success");
    assert_eq!(response.message, "Transfer retrieved successfully");

    let data = response.data;

    assert_eq!(data.len(), 2);

    assert_eq!(data[0].transfer_id, 1);
    assert_eq!(data[0].transfer_from, 1);
    assert_eq!(data[0].transfer_to, 2);
    assert_eq!(data[0].transfer_amount, 10000);
}

#[tokio::test]
async fn test_find_by_id_success() {
    let mut mock_transfer_repo = MockTransferRepositoryTrait::new();
    let mut _mock_saldo_repo = MockSaldoRepositoryTrait::new();
    let mut _mock_user_repo = MockUserRepositoryTrait::new();

    let mock_transfer = transfers::Model {
        transfer_id: 1,
        transfer_from: 1,
        transfer_to: 2,
        transfer_amount: 10000,
        transfer_time: DateTime::<Utc>::from(Utc::now()).naive_utc(),
        created_at: Some(Utc::now().naive_utc()),
        updated_at: Some(Utc::now().naive_utc()),
    };

    mock_transfer_repo
        .expect_find_by_id()
        .with(predicate::eq(1))
        .return_once(move |_| Ok(Some(mock_transfer.clone())));

    let transfer_service = TransferService::new(
        Arc::new(mock_transfer_repo),
        Arc::new(_mock_saldo_repo),
        Arc::new(_mock_user_repo),
    );

    let result = transfer_service.get_transfer(1).await;

    assert!(result.is_ok());

    let response = result.unwrap();

    assert_eq!(response.status, "success");
    assert_eq!(response.message, "Transfer retrieved successfully");

    let data = response.data.unwrap();

    assert_eq!(data.transfer_id, 1);
    assert_eq!(data.transfer_from, 1);
    assert_eq!(data.transfer_to, 2);
    assert_eq!(data.transfer_amount, 10000);
}

#[tokio::test]
async fn test_find_by_id_not_found() {
    let mut mock_transfer_repo = MockTransferRepositoryTrait::new();
    let mut _mock_saldo_repo = MockSaldoRepositoryTrait::new();
    let mut _mock_user_repo = MockUserRepositoryTrait::new();

    mock_transfer_repo
        .expect_find_by_id()
        .with(predicate::eq(1))
        .return_once(move |_| Ok(None));

    let transfer_service = TransferService::new(
        Arc::new(mock_transfer_repo),
        Arc::new(_mock_saldo_repo),
        Arc::new(_mock_user_repo),
    );

    let result = transfer_service.get_transfer(1).await;

    assert!(result.is_err());

    let response = result.unwrap_err();

    assert_eq!(response.status, "error");
    assert_eq!(response.message, "Transfer with id 1 not found");
}

#[tokio::test]
async fn test_find_transfer_users() {
    let mut mock_transfer_repo = MockTransferRepositoryTrait::new();
    let mut _mock_saldo_repo = MockSaldoRepositoryTrait::new();
    let mut _mock_user_repo = MockUserRepositoryTrait::new();

    let mock_transfers = vec![
        transfers::Model {
            transfer_id: 1,
            transfer_from: 1,
            transfer_to: 2,
            transfer_amount: 10000,
            transfer_time: DateTime::<Utc>::from(Utc::now()).naive_utc(),
            created_at: Some(Utc::now().naive_utc()),
            updated_at: Some(Utc::now().naive_utc()),
        },
        transfers::Model {
            transfer_id: 2,
            transfer_from: 2,
            transfer_to: 1,
            transfer_amount: 5000,
            transfer_time: DateTime::<Utc>::from(Utc::now()).naive_utc(),
            created_at: Some(Utc::now().naive_utc()),
            updated_at: Some(Utc::now().naive_utc()),
        },
    ];

    mock_transfer_repo
        .expect_find_all()
        .return_once(move || Ok(mock_transfers.clone()));

    let transfer_service = TransferService::new(
        Arc::new(mock_transfer_repo),
        Arc::new(_mock_saldo_repo),
        Arc::new(_mock_user_repo),
    );

    let result = transfer_service.get_transfers().await;

    assert!(result.is_ok());

    let response = result.unwrap();

    assert_eq!(response.status, "success");
    assert_eq!(response.message, "Transfer retrieved successfully");

    let data = response.data;

    assert_eq!(data.len(), 2);

    assert_eq!(data[0].transfer_id, 1);
    assert_eq!(data[0].transfer_from, 1);
    assert_eq!(data[0].transfer_to, 2);
    assert_eq!(data[0].transfer_amount, 10000);
}

#[tokio::test]
async fn test_find_transfer_users_not_found() {
    let mut mock_transfer_repo = MockTransferRepositoryTrait::new();
    let mut _mock_saldo_repo = MockSaldoRepositoryTrait::new();
    let mut _mock_user_repo = MockUserRepositoryTrait::new();

    mock_transfer_repo
        .expect_find_all()
        .return_once(move || Ok(vec![]));

    let transfer_service = TransferService::new(
        Arc::new(mock_transfer_repo),
        Arc::new(_mock_saldo_repo),
        Arc::new(_mock_user_repo),
    );

    let result = transfer_service.get_transfers().await;

    assert!(result.is_ok());

    let response = result.unwrap();
    assert_eq!(response.status, "success");
    assert_eq!(response.message, "Transfer retrieved successfully");
    assert!(response.data.is_empty());
}

#[tokio::test]
async fn test_get_transfer_user_success() {
    let mut mock_user_repo = MockUserRepositoryTrait::new();
    let mut mock_transfer_repo = MockTransferRepositoryTrait::new();
    let user_id = 1;

    let mock_user = users::Model {
        user_id,
        firstname: "John".to_string(),
        lastname: "Doe".to_string(),
        email: "john.doe@example.com".to_string(),
        password: "hashed_password".to_string(),
        noc_transfer: "12345".to_string(),
        created_at: Some(Utc::now().naive_utc()),
        updated_at: Some(Utc::now().naive_utc()),
    };

    let mock_transfer = transfers::Model {
        transfer_id: 1,
        transfer_from: user_id,
        transfer_to: 2,
        transfer_amount: 10000,
        transfer_time: DateTime::<Utc>::from(Utc::now()).naive_utc(),
        created_at: Some(Utc::now().naive_utc()),
        updated_at: Some(Utc::now().naive_utc()),
    };

    let mock_transfer_for_closure = mock_transfer.clone();
    let mock_transfer_for_assertion = mock_transfer.clone();

    mock_user_repo
        .expect_find_by_id()
        .with(predicate::eq(user_id))
        .return_once(move |_| Ok(Some(mock_user)));

    mock_transfer_repo
        .expect_find_by_user()
        .with(predicate::eq(user_id))
        .return_once(move |_| Ok(Some(mock_transfer_for_closure)));

    let transfer_service = TransferService::new(
        Arc::new(mock_transfer_repo),
        Arc::new(MockSaldoRepositoryTrait::new()),
        Arc::new(mock_user_repo),
    );

    // Execute test
    let result = transfer_service.get_transfer_user(user_id).await;

    // Assertions
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.status, "success");
    assert_eq!(response.message, "Success");

    let data = response.data.unwrap();
    assert_eq!(data.transfer_id, mock_transfer_for_assertion.transfer_id);
    assert_eq!(
        data.transfer_from,
        mock_transfer_for_assertion.transfer_from
    );
    assert_eq!(data.transfer_to, mock_transfer_for_assertion.transfer_to);
    assert_eq!(
        data.transfer_amount,
        mock_transfer_for_assertion.transfer_amount
    );
}

#[tokio::test]
async fn test_get_transfer_user_not_found() {
    let mut mock_user_repo = MockUserRepositoryTrait::new();
    let mut mock_transfer_repo = MockTransferRepositoryTrait::new();
    let user_id = 1;

    let mock_user = users::Model {
        user_id,
        firstname: "John".to_string(),
        lastname: "Doe".to_string(),
        email: "john.doe@example.com".to_string(),
        password: "hashed_password".to_string(),
        noc_transfer: "12345".to_string(),
        created_at: Some(Utc::now().naive_utc()),
        updated_at: Some(Utc::now().naive_utc()),
    };

    // Setup mock expectations
    mock_user_repo
        .expect_find_by_id()
        .with(predicate::eq(user_id))
        .return_once(move |_| Ok(Some(mock_user)));

    mock_transfer_repo
        .expect_find_by_user()
        .with(predicate::eq(user_id))
        .return_once(move |_| Ok(None));

    let transfer_service = TransferService::new(
        Arc::new(mock_transfer_repo),
        Arc::new(MockSaldoRepositoryTrait::new()),
        Arc::new(mock_user_repo),
    );

    // Execute test
    let result = transfer_service.get_transfer_user(user_id).await;

    // Assertions
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.status, "success");
    assert_eq!(response.message, format!("Success"));
    assert!(response.data.is_none());
}

#[tokio::test]
async fn test_create_transfer_success() {
    let mut mock_user_repo = MockUserRepositoryTrait::new();
    let mut mock_transfer_repo = MockTransferRepositoryTrait::new();
    let mut mock_saldo_repo = MockSaldoRepositoryTrait::new();

    let sender_id = 1;
    let receiver_id = 2;
    let transfer_amount = 50000000;
    let sender_initial_balance = 500000;
    let receiver_initial_balance = 500000;

    let create_request = CreateTransferRequest {
        transfer_from: sender_id,
        transfer_to: receiver_id,
        transfer_amount,
    };

    mock_user_repo
        .expect_find_by_id()
        .with(predicate::eq(sender_id))
        .return_once(move |_| {
            Ok(Some(users::Model {
                user_id: sender_id,
                firstname: "Sender".to_string(),
                lastname: "User".to_string(),
                email: "sender@test.com".to_string(),
                password: "hash".to_string(),
                noc_transfer: "12345".to_string(),
                created_at: Some(Utc::now().naive_utc()),
                updated_at: Some(Utc::now().naive_utc()),
            }))
        });

    mock_user_repo
        .expect_find_by_id()
        .with(predicate::eq(receiver_id))
        .return_once(move |_| {
            Ok(Some(users::Model {
                user_id: receiver_id,
                firstname: "Receiver".to_string(),
                lastname: "User".to_string(),
                email: "receiver@test.com".to_string(),
                password: "hash".to_string(),
                noc_transfer: "67890".to_string(),
                created_at: Some(Utc::now().naive_utc()),
                updated_at: Some(Utc::now().naive_utc()),
            }))
        });

    let expected_transfer = transfers::Model {
        transfer_id: 1,
        transfer_from: sender_id,
        transfer_to: receiver_id,
        transfer_amount,
        transfer_time: Utc::now().naive_utc(),
        created_at: Some(Utc::now().naive_utc()),
        updated_at: Some(Utc::now().naive_utc()),
    };

    let expected_transfer_clone = expected_transfer.clone();

    mock_transfer_repo
        .expect_create()
        .withf(move |req| {
            req.transfer_from == create_request.transfer_from
                && req.transfer_to == create_request.transfer_to
                && req.transfer_amount == create_request.transfer_amount
        })
        .return_once(move |_| Ok(expected_transfer_clone));

    mock_saldo_repo
        .expect_find_by_user_id()
        .with(predicate::eq(sender_id))
        .return_once(move |_| {
            Ok(Some(saldo::Model {
                saldo_id: 1,
                user_id: sender_id,
                total_balance: sender_initial_balance,
                withdraw_amount: None,
                withdraw_time: None,
                created_at: Some(Utc::now().naive_utc()),
                updated_at: Some(Utc::now().naive_utc()),
            }))
        });

    mock_saldo_repo
        .expect_find_by_user_id()
        .with(predicate::eq(receiver_id))
        .return_once(move |_| {
            Ok(Some(saldo::Model {
                saldo_id: 2,
                user_id: receiver_id,
                total_balance: receiver_initial_balance,
                withdraw_amount: None,
                withdraw_time: None,
                created_at: Some(Utc::now().naive_utc()),
                updated_at: Some(Utc::now().naive_utc()),
            }))
        });

    mock_saldo_repo
        .expect_update_balance()
        .withf(move |req| {
            req.user_id == sender_id
                && req.total_balance == sender_initial_balance - transfer_amount
        })
        .return_once(move |_| {
            Ok(saldo::Model {
                saldo_id: 1,
                user_id: sender_id,
                total_balance: sender_initial_balance - transfer_amount,
                withdraw_amount: None,
                withdraw_time: None,
                created_at: Some(Utc::now().naive_utc()),
                updated_at: Some(Utc::now().naive_utc()),
            })
        });

    mock_saldo_repo
        .expect_update_balance()
        .withf(move |req| {
            req.user_id == receiver_id
                && req.total_balance == receiver_initial_balance + transfer_amount
        })
        .return_once(move |_| {
            Ok(saldo::Model {
                saldo_id: 2,
                user_id: receiver_id,
                total_balance: receiver_initial_balance + transfer_amount,
                withdraw_amount: None,
                withdraw_time: None,
                created_at: Some(Utc::now().naive_utc()),
                updated_at: Some(Utc::now().naive_utc()),
            })
        });

    let service = TransferService::new(
        Arc::new(mock_transfer_repo),
        Arc::new(mock_saldo_repo),
        Arc::new(mock_user_repo),
    );

    let result = service.create_transfer(&create_request).await;

    let response = result.unwrap();
    assert_eq!(response.status, "success");
    assert_eq!(response.message, "Transfer created successfully");
    let data = response.data;
    assert_eq!(data.transfer_id, expected_transfer.transfer_id);
    assert_eq!(data.transfer_from, expected_transfer.transfer_from);
    assert_eq!(data.transfer_to, expected_transfer.transfer_to);
    assert_eq!(data.transfer_amount, expected_transfer.transfer_amount);
}

#[tokio::test]
async fn test_update_transfer_success() {
    let mut _mock_user_repo = MockUserRepositoryTrait::new();
    let mut mock_transfer_repo = MockTransferRepositoryTrait::new();
    let mut mock_saldo_repo = MockSaldoRepositoryTrait::new();

    let transfer_id = 1;
    let sender_id = 1;
    let receiver_id = 2;
    let original_transfer_amount: i32 = 5000000;
    let new_transfer_amount = 5000000;
    let sender_initial_balance = 100000;
    let receiver_initial_balance = 100000;

    let update_request = UpdateTransferRequest {
        transfer_id,
        transfer_from: sender_id,
        transfer_to: receiver_id,
        transfer_amount: new_transfer_amount,
    };

    let existing_transfer = transfers::Model {
        transfer_id,
        transfer_from: sender_id,
        transfer_to: receiver_id,
        transfer_amount: original_transfer_amount,
        transfer_time: Utc::now().naive_utc(),
        created_at: Some(Utc::now().naive_utc()),
        updated_at: Some(Utc::now().naive_utc()),
    };

    mock_transfer_repo
        .expect_find_by_id()
        .with(predicate::eq(transfer_id))
        .return_once(move |_| Ok(Some(existing_transfer.clone())));

    // Setup sender saldo mock
    mock_saldo_repo
        .expect_find_by_user_id()
        .with(predicate::eq(sender_id))
        .return_once(move |_| {
            Ok(Some(saldo::Model {
                saldo_id: 1,
                user_id: sender_id,
                total_balance: sender_initial_balance,
                withdraw_amount: None,
                withdraw_time: None,
                created_at: Some(Utc::now().naive_utc()),
                updated_at: Some(Utc::now().naive_utc()),
            }))
        });

    mock_saldo_repo
        .expect_find_by_user_id()
        .with(predicate::eq(receiver_id))
        .return_once(move |_| {
            Ok(Some(saldo::Model {
                saldo_id: 2,
                user_id: receiver_id,
                total_balance: receiver_initial_balance,
                withdraw_amount: None,
                withdraw_time: None,
                created_at: Some(Utc::now().naive_utc()),
                updated_at: Some(Utc::now().naive_utc()),
            }))
        });

    let amount_difference = new_transfer_amount - original_transfer_amount;
    let new_sender_balance = sender_initial_balance - amount_difference;
    let new_receiver_balance = receiver_initial_balance + amount_difference;

    mock_saldo_repo
        .expect_update_balance()
        .withf(move |req| req.user_id == sender_id && req.total_balance == new_sender_balance)
        .return_once(move |_| {
            Ok(saldo::Model {
                saldo_id: 1,
                user_id: sender_id,
                total_balance: new_sender_balance,
                withdraw_amount: None,
                withdraw_time: None,
                created_at: Some(Utc::now().naive_utc()),
                updated_at: Some(Utc::now().naive_utc()),
            })
        });

    mock_saldo_repo
        .expect_update_balance()
        .withf(move |req| req.user_id == receiver_id && req.total_balance == new_receiver_balance)
        .return_once(move |_| {
            Ok(saldo::Model {
                saldo_id: 2,
                user_id: receiver_id,
                total_balance: new_receiver_balance,
                withdraw_amount: None,
                withdraw_time: None,
                created_at: Some(Utc::now().naive_utc()),
                updated_at: Some(Utc::now().naive_utc()),
            })
        });

    let updated_transfer = transfers::Model {
        transfer_id,
        transfer_from: sender_id,
        transfer_to: receiver_id,
        transfer_amount: new_transfer_amount,
        transfer_time: Utc::now().naive_utc(),
        created_at: Some(Utc::now().naive_utc()),
        updated_at: Some(Utc::now().naive_utc()),
    };

    mock_transfer_repo
        .expect_update()
        .withf(move |req| {
            req.transfer_id == transfer_id && req.transfer_amount == new_transfer_amount
        })
        .return_once(move |_| Ok(updated_transfer.clone()));

    let service = TransferService::new(
        Arc::new(mock_transfer_repo),
        Arc::new(mock_saldo_repo),
        Arc::new(_mock_user_repo),
    );

    let result = service.update_transfer(&update_request).await;

    let response = result.unwrap();
    assert_eq!(response.status, "success");
    assert_eq!(response.message, "Transfer updated successfully");
    let data = response.data;
    assert_eq!(data.transfer_id, transfer_id);
    assert_eq!(data.transfer_from, sender_id);
    assert_eq!(data.transfer_to, receiver_id);
    assert_eq!(data.transfer_amount, new_transfer_amount);
}

#[tokio::test]
async fn test_delete_transfer_success() {
    let mut mock_user_repo = MockUserRepositoryTrait::new();
    let mut mock_transfer_repo = MockTransferRepositoryTrait::new();
    let user_id = 1;

    let mock_user = users::Model {
        user_id,
        firstname: "John".to_string(),
        lastname: "Doe".to_string(),
        email: "john.doe@example.com".to_string(),
        password: "hashed_password".to_string(),
        noc_transfer: "12345".to_string(),
        created_at: Some(Utc::now().naive_utc()),
        updated_at: Some(Utc::now().naive_utc()),
    };

    let mock_transfer = transfers::Model {
        transfer_id: 1,
        transfer_from: user_id,
        transfer_to: 2,
        transfer_amount: 10000,
        transfer_time: DateTime::<Utc>::from(Utc::now()).naive_utc(),
        created_at: Some(Utc::now().naive_utc()),
        updated_at: Some(Utc::now().naive_utc()),
    };

    let mock_transfer_id = mock_transfer.transfer_id.clone();

    mock_user_repo
        .expect_find_by_id()
        .with(predicate::eq(user_id))
        .return_once(move |_| Ok(Some(mock_user)));

    mock_transfer_repo
        .expect_find_by_user()
        .with(predicate::eq(user_id))
        .return_once(move |_| Ok(Some(mock_transfer.clone())));

    mock_transfer_repo
        .expect_delete()
        .with(predicate::eq(mock_transfer_id))
        .return_once(move |_| Ok(()));

    let service = TransferService::new(
        Arc::new(mock_transfer_repo),
        Arc::new(MockSaldoRepositoryTrait::new()),
        Arc::new(mock_user_repo),
    );

    let result = service.delete_transfer(user_id).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.status, "success");
    assert_eq!(response.message, "Topup deleted successfully");
}

#[tokio::test]
async fn test_delete_transfer_not_found() {
    let mut mock_transfer_repo = MockTransferRepositoryTrait::new();
    let mut mock_user_repo = MockUserRepositoryTrait::new();
    let user_id = 1;

    mock_user_repo
        .expect_find_by_id()
        .with(predicate::eq(user_id))
        .return_once(move |_| {
            Ok(Some(users::Model {
                user_id,
                firstname: "Test".to_string(),
                lastname: "User".to_string(),
                email: "test@example.com".to_string(),
                password: "hash".to_string(),
                noc_transfer: "12345".to_string(),
                created_at: Some(Utc::now().naive_utc()),
                updated_at: Some(Utc::now().naive_utc()),
            }))
        });

    mock_transfer_repo
        .expect_find_by_user()
        .with(predicate::eq(user_id))
        .return_once(|_| Ok(None));

    let service = TransferService::new(
        Arc::new(mock_transfer_repo),
        Arc::new(MockSaldoRepositoryTrait::new()),
        Arc::new(mock_user_repo),
    );

    let result = service.delete_transfer(user_id).await;

    assert!(result.is_err());
    let error = result.unwrap_err();
    assert_eq!(error.status, "error");
    assert_eq!(
        error.message,
        format!("Topup with id {} not found", user_id)
    );
}
