use crate::{
    abstract_trait::{
        saldo::DynSaldoRepository,
        user::DynUserRepository,
        withdraw::{DynWithdrawRepository, WithdrawServiceTrait},
    },
    domain::{
        request::{
            saldo::UpdateSaldoBalance,
            withdraw::{CreateWithdrawRequest, UpdateWithdrawRequest},
        },
        response::{withdraw::WithdrawResponse, ApiResponse, ErrorResponse},
    },
    utils::errors::AppError,
};
use async_trait::async_trait;
use chrono::Utc;
use tracing::{error, info};

pub struct WithdrawService {
    withdraw_repository: DynWithdrawRepository,
    saldo_repository: DynSaldoRepository,
    user_repository: DynUserRepository,
}

impl WithdrawService {
    pub fn new(
        withdraw_repository: DynWithdrawRepository,
        saldo_repository: DynSaldoRepository,
        user_repository: DynUserRepository,
    ) -> Self {
        Self {
            withdraw_repository,
            saldo_repository,
            user_repository,
        }
    }
}

#[async_trait]
impl WithdrawServiceTrait for WithdrawService {
    async fn get_withdraws(&self) -> Result<ApiResponse<Vec<WithdrawResponse>>, ErrorResponse> {
        let withdraw = self
            .withdraw_repository
            .find_all()
            .await
            .map_err(AppError::from)
            .map_err(ErrorResponse::from)?;

        let withdraw_response: Vec<WithdrawResponse> = withdraw
            .into_iter()
            .map(|withdraw| WithdrawResponse::from(withdraw))
            .collect();

        Ok(ApiResponse {
            status: "success".to_string(),
            message: "Withdraw retrieved successfully".to_string(),
            data: withdraw_response,
        })
    }

    async fn get_withdraw(
        &self,
        id: i32,
    ) -> Result<ApiResponse<Option<WithdrawResponse>>, ErrorResponse> {
        let withdraw = self
            .withdraw_repository
            .find_by_id(id)
            .await
            .map_err(AppError::from)
            .map_err(ErrorResponse::from)?;

        if let Some(withdraw) = withdraw {
            Ok(ApiResponse {
                status: "success".to_string(),
                message: "Withdraw retrieved successfully".to_string(),
                data: Some(WithdrawResponse::from(withdraw)),
            })
        } else {
            Err(ErrorResponse::from(AppError::NotFound(format!(
                "Saldo with id {} not found",
                id
            ))))
        }
    }

    async fn get_withdraw_users(
        &self,
        id: i32,
    ) -> Result<ApiResponse<Option<Vec<WithdrawResponse>>>, ErrorResponse> {
        let _user = self.user_repository.find_by_id(id).await.map_err(|_| {
            ErrorResponse::from(AppError::NotFound(format!("User with id {} not found", id)))
        })?;
    
        
        let transfer = self
            .withdraw_repository
            .find_by_users(id)
            .await
            .map_err(AppError::from)
            .map_err(ErrorResponse::from)?;
    
       
        let transfer_response: Option<Vec<WithdrawResponse>> = transfer.map(|transfers| {
            transfers
                .into_iter()
                .map(WithdrawResponse::from)
                .collect()
        });
    
        let response = ApiResponse {
            status: "success".to_string(),
            data: transfer_response,
            message: "Success".to_string(),
        };
    
        Ok(response)
    }
    
    async fn get_withdraw_user(
        &self,
        id: i32,
    ) -> Result<ApiResponse<Option<WithdrawResponse>>, ErrorResponse> {
        let _user = self.user_repository.find_by_id(id).await.map_err(|_| {
            ErrorResponse::from(AppError::NotFound(format!("User with id {} not found", id)))
        })?;

        let withdraw: Option<WithdrawResponse> = self
            .withdraw_repository
            .find_by_user(id)
            .await
            .map_err(AppError::from)
            .map_err(ErrorResponse::from)?
            .map(WithdrawResponse::from);

        // Create the response object
        let response = ApiResponse {
            status: "success".to_string(),
            data: withdraw,
            message: "Success".to_string(),
        };

        Ok(response)
    }

    async fn create_withdraw(
        &self,
        input: &CreateWithdrawRequest,
    ) -> Result<ApiResponse<WithdrawResponse>, ErrorResponse> {
        if let Err(validation_err) = input.validate() {
            error!("Validation failed for topup create: {}", validation_err);
            return Err(ErrorResponse::from(AppError::ValidationError(
                validation_err,
            )));
        }

        let saldo = self
            .saldo_repository
            .find_by_user_id(input.user_id)
            .await
            .map_err(|_| {
                ErrorResponse::from(AppError::NotFound(format!(
                    "Saldo with user_id {} not found",
                    input.user_id
                )))
            })?;

        
        let saldo_ref = saldo.as_ref().ok_or_else(|| {
            ErrorResponse::from(AppError::NotFound("Saldo not found".to_string()))
        })?;

        let new_total_balance = saldo_ref.total_balance - input.withdraw_amount;

        let _update_saldo_balance = self
            .saldo_repository
            .update_balance(&UpdateSaldoBalance {
                user_id: input.user_id,
                withdraw_amount: Some(input.withdraw_amount),
                withdraw_time: Some(Utc::now().naive_utc()),
                total_balance: new_total_balance,
            })
            .await
            .map_err(AppError::from)
            .map_err(ErrorResponse::from)?;

        let withdraw_create_result = self
            .withdraw_repository
            .create(&input)
            .await
            .map_err(AppError::from)
            .map_err(ErrorResponse::from)?;

        match withdraw_create_result {
            withdraw => {
                let _update_saldo = self
                    .saldo_repository
                    .update_balance(&UpdateSaldoBalance {
                        user_id: input.user_id,
                        withdraw_amount: Some(input.withdraw_amount),
                        withdraw_time: Some(Utc::now().naive_utc()),
                        total_balance: saldo_ref.total_balance, // No move here
                    })
                    .await
                    .map_err(AppError::from)
                    .map_err(ErrorResponse::from)?;

                Ok(ApiResponse {
                    status: "success".to_string(),
                    message: "Withdraw created successfully".to_string(),
                    data: withdraw.into(),
                })
            }
        }
    }

    async fn update_withdraw(
        &self,
        input: &UpdateWithdrawRequest,
    ) -> Result<ApiResponse<Option<WithdrawResponse>>, ErrorResponse> {
        if let Err(validation_err) = input.validate() {
            error!("Validation failed for withdraw update: {}", validation_err);
            return Err(ErrorResponse::from(AppError::ValidationError(
                validation_err,
            )));
        }

        let _withdraw = self
            .withdraw_repository
            .find_by_id(input.withdraw_id) // Assuming `withdraw_id` exists in the request
            .await
            .map_err(|_| {
                ErrorResponse::from(AppError::NotFound(format!(
                    "Withdraw with id {} not found",
                    input.withdraw_id
                )))
            })?;

        let saldo = self
            .saldo_repository
            .find_by_user_id(input.user_id)
            .await
            .map_err(|_| {
                ErrorResponse::from(AppError::NotFound(format!(
                    "Saldo with user_id {} not found",
                    input.user_id
                )))
            })?;

        let saldo_ref = saldo.as_ref().ok_or_else(|| {
            ErrorResponse::from(AppError::NotFound("Saldo not found".to_string()))
        })?;

        let new_total_balance = saldo_ref.total_balance - input.withdraw_amount;

       
        let updated_withdraw = self
            .withdraw_repository
            .update(&input) 
            .await
            .map_err(AppError::from)
            .map_err(ErrorResponse::from);

      
        if let Err(err) = updated_withdraw {
           
            let _rollback_saldo = self
                .saldo_repository
                .update_balance(&UpdateSaldoBalance {
                    user_id: input.user_id,
                    withdraw_amount: None, 
                    withdraw_time: None,  
                    total_balance: saldo_ref.total_balance,
                })
                .await
                .map_err(AppError::from)
                .map_err(ErrorResponse::from)?;

            
            error!("Rollback: Saldo reverted due to withdraw update failure");

           
            return Err(err);
        }

      
        let _update_saldo = self
            .saldo_repository
            .update_balance(&UpdateSaldoBalance {
                user_id: input.user_id,
                withdraw_amount: Some(input.withdraw_amount),
                withdraw_time: Some(Utc::now().naive_utc()), 
                total_balance: new_total_balance,
            })
            .await
            .map_err(AppError::from)
            .map_err(ErrorResponse::from)?;

        Ok(ApiResponse {
            status: "success".to_string(),
            message: "Withdraw updated successfully".to_string(),
            data: Some(updated_withdraw.unwrap().into()), 
        })
    }

    async fn delete_withdraw(&self, id: i32) -> Result<ApiResponse<()>, ErrorResponse> {
        let user = self.user_repository.find_by_id(id).await.map_err(|_| {
            ErrorResponse::from(AppError::NotFound(format!("User with id {} not found", id)))
        })?;

        let existing = self
            .withdraw_repository
            .find_by_user(user.unwrap().user_id)
            .await
            .map_err(AppError::from)
            .map_err(ErrorResponse::from)?;

        match existing {
            Some(_) => {
                self.withdraw_repository
                    .delete(existing.unwrap().withdraw_id)
                    .await
                    .map_err(AppError::from)
                    .map_err(ErrorResponse::from)?;

                info!("Withdraw deleted successfully for id: {}", id);

                Ok(ApiResponse {
                    status: "success".to_string(),
                    message: "Withdraw deleted successfully".to_string(),
                    data: (),
                })
            }
            None => {
                error!("Withdraw with id {} not found", id);
                Err(ErrorResponse::from(AppError::NotFound(format!(
                    "Withdraw with id {} not found",
                    id
                ))))
            }
        }
    }
}
