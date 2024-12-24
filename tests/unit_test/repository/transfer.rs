use chrono::{DateTime, Utc};
use example_payment_gateway_axum::{
    abstract_trait::transfer::{MockTransferRepositoryTrait, TransferRepositoryTrait},
    domain::request::transfer::{
        CreateTransferRequest, UpdateTransferAmountRequest, UpdateTransferRequest,
    },
    entities::transfers,
};
use mockall::predicate::*;
use tokio;

#[tokio::test]
async fn test_find_all_transfers() {
    let mut mock_repo = MockTransferRepositoryTrait::new();

    let mock_transfer = transfers::Model {
        transfer_id: 1,
        transfer_from: 1001,
        transfer_to: 1002,
        transfer_amount: 5000,
        transfer_time: DateTime::<Utc>::from(Utc::now()).naive_utc(),
        created_at: Some(DateTime::<Utc>::from(Utc::now()).naive_utc()),
        updated_at: Some(DateTime::<Utc>::from(Utc::now()).naive_utc()),
    };

    mock_repo
        .expect_find_all()
        .returning(move || Ok(vec![mock_transfer.clone()]));

    let result = mock_repo.find_all().await;

    assert!(result.is_ok());
    let transfers = result.unwrap();
    assert_eq!(transfers.len(), 1);
    assert_eq!(transfers[0].transfer_id, 1);
}

#[tokio::test]
async fn test_find_transfer_by_id() {
    let mut mock_repo = MockTransferRepositoryTrait::new();

    let mock_transfer = transfers::Model {
        transfer_id: 1,
        transfer_from: 1001,
        transfer_to: 1002,
        transfer_amount: 5000,
        transfer_time: DateTime::<Utc>::from(Utc::now()).naive_utc(),
        created_at: Some(DateTime::<Utc>::from(Utc::now()).naive_utc()),
        updated_at: Some(DateTime::<Utc>::from(Utc::now()).naive_utc()),
    };

    mock_repo
        .expect_find_by_id()
        .with(eq(1))
        .returning(move |_| Ok(Some(mock_transfer.clone())));

    let result = mock_repo.find_by_id(1).await;

    assert!(result.is_ok());
    let transfer = result.unwrap();
    assert!(transfer.is_some());
    assert_eq!(transfer.unwrap().transfer_id, 1);
}

#[tokio::test]
async fn test_create_transfer() {
    let mut mock_repo = MockTransferRepositoryTrait::new();

    let mock_request = CreateTransferRequest {
        transfer_from: 1001,
        transfer_to: 1002,
        transfer_amount: 5000,
    };

    let mock_transfer = transfers::Model {
        transfer_id: 1,
        transfer_from: 1001,
        transfer_to: 1002,
        transfer_amount: 5000,
        transfer_time: DateTime::<Utc>::from(Utc::now()).naive_utc(),
        created_at: Some(DateTime::<Utc>::from(Utc::now()).naive_utc()),
        updated_at: Some(DateTime::<Utc>::from(Utc::now()).naive_utc()),
    };

    mock_repo
        .expect_create()
        .withf(move |input| {
            input.transfer_from == mock_request.transfer_from
                && input.transfer_to == mock_request.transfer_to
                && input.transfer_amount == mock_request.transfer_amount
        })
        .returning(move |_| Ok(mock_transfer.clone()));

    let result = mock_repo.create(&mock_request).await;

    assert!(result.is_ok());
    let transfer = result.unwrap();
    assert_eq!(transfer.transfer_id, 1);
    assert_eq!(transfer.transfer_amount, 5000);
}

#[tokio::test]
async fn test_update_transfer() {
    let mut mock_repo = MockTransferRepositoryTrait::new();

    let mock_request = UpdateTransferRequest {
        transfer_id: 1,
        transfer_from: 1001,
        transfer_to: 1002,
        transfer_amount: 7000,
    };

    let mock_transfer = transfers::Model {
        transfer_id: 1,
        transfer_from: 1001,
        transfer_to: 1002,
        transfer_amount: 7000,
        transfer_time: DateTime::<Utc>::from(Utc::now()).naive_utc(),
        created_at: Some(DateTime::<Utc>::from(Utc::now()).naive_utc()),
        updated_at: Some(DateTime::<Utc>::from(Utc::now()).naive_utc()),
    };

    mock_repo
        .expect_update()
        .withf(move |input| {
            input.transfer_id == mock_request.transfer_id
                && input.transfer_from == mock_request.transfer_from
                && input.transfer_to == mock_request.transfer_to
                && input.transfer_amount == mock_request.transfer_amount
        })
        .returning(move |_| Ok(mock_transfer.clone()));

    let result = mock_repo.update(&mock_request).await;

    assert!(result.is_ok());
    let updated_transfer = result.unwrap();
    assert_eq!(updated_transfer.transfer_id, 1);
    assert_eq!(updated_transfer.transfer_from, 1001);
    assert_eq!(updated_transfer.transfer_to, 1002);
    assert_eq!(updated_transfer.transfer_amount, 7000);
}

#[tokio::test]
async fn test_update_transfer_amount() {
    let mut mock_repo = MockTransferRepositoryTrait::new();

    let mock_request = UpdateTransferAmountRequest {
        transfer_id: 1,
        transfer_amount: 7000,
    };

    let mock_transfer = transfers::Model {
        transfer_id: 1,
        transfer_from: 1001,
        transfer_to: 1002,
        transfer_amount: 7000,
        transfer_time: DateTime::<Utc>::from(Utc::now()).naive_utc(),
        created_at: Some(DateTime::<Utc>::from(Utc::now()).naive_utc()),
        updated_at: Some(DateTime::<Utc>::from(Utc::now()).naive_utc()),
    };

    mock_repo
        .expect_update_amount()
        .withf(move |input| input.transfer_id == mock_request.transfer_id)
        .returning(move |_| Ok(mock_transfer.clone()));

    let result = mock_repo.update_amount(&mock_request).await;

    assert!(result.is_ok());
    let transfer = result.unwrap();
    assert_eq!(transfer.transfer_id, 1);
    assert_eq!(transfer.transfer_amount, 7000);
}

#[tokio::test]
async fn test_delete_transfer() {
    let mut mock_repo = MockTransferRepositoryTrait::new();

    mock_repo.expect_delete().with(eq(1)).returning(|_| Ok(()));

    let result = mock_repo.delete(1).await;

    assert!(result.is_ok());
}
