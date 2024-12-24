use async_trait::async_trait;
use mockall::automock;
use sea_orm::DbErr;
use std::sync::Arc;

use crate::{
    domain::{
        request::transfer::{
            CreateTransferRequest, UpdateTransferAmountRequest, UpdateTransferRequest,
        },
        response::{transfer::TransferResponse, ApiResponse, ErrorResponse},
    },
    entities::transfers,
};

pub type DynTransferRepository = Arc<dyn TransferRepositoryTrait + Send + Sync>;
pub type DynTransferService = Arc<dyn TransferServiceTrait + Send + Sync>;

#[automock]
#[async_trait]
pub trait TransferRepositoryTrait {
    async fn find_all(&self) -> Result<Vec<transfers::Model>, DbErr>;
    async fn find_by_id(&self, id: i32) -> Result<Option<transfers::Model>, DbErr>;
    async fn find_by_users(&self, id: i32) -> Result<Option<Vec<transfers::Model>>, DbErr>;
    async fn find_by_user(&self, id: i32) -> Result<Option<transfers::Model>, DbErr>;
    async fn create(&self, input: &CreateTransferRequest) -> Result<transfers::Model, DbErr>;
    async fn update(&self, input: &UpdateTransferRequest) -> Result<transfers::Model, DbErr>;
    async fn update_amount(
        &self,
        input: &UpdateTransferAmountRequest,
    ) -> Result<transfers::Model, DbErr>;
    async fn delete(&self, id: i32) -> Result<(), DbErr>;
}

#[automock]
#[async_trait]
pub trait TransferServiceTrait {
    async fn get_transfers(&self) -> Result<ApiResponse<Vec<TransferResponse>>, ErrorResponse>;
    async fn get_transfer(
        &self,
        id: i32,
    ) -> Result<ApiResponse<Option<TransferResponse>>, ErrorResponse>;
    async fn get_transfer_users(
        &self,
        id: i32,
    ) -> Result<ApiResponse<Option<Vec<TransferResponse>>>, ErrorResponse>;
    async fn get_transfer_user(
        &self,
        id: i32,
    ) -> Result<ApiResponse<Option<TransferResponse>>, ErrorResponse>;
    async fn create_transfer(
        &self,
        input: &CreateTransferRequest,
    ) -> Result<ApiResponse<TransferResponse>, ErrorResponse>;
    async fn update_transfer(
        &self,
        input: &UpdateTransferRequest,
    ) -> Result<ApiResponse<TransferResponse>, ErrorResponse>;
    async fn delete_transfer(&self, id: i32) -> Result<ApiResponse<()>, ErrorResponse>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{DateTime, Utc};
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
}
