use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CreateSaldoRequest {
    #[serde(rename = "user_id")]
    pub user_id: i32,

    #[serde(rename = "total_balance")]
    pub total_balance: i32,
}

impl CreateSaldoRequest {
    pub fn validate(&self) -> Result<(), String> {
        if self.user_id <= 0 {
            return Err("User ID must be greater than 0".to_string());
        }

        if self.total_balance < 50000 {
            return Err("total balance must be greater than or equal to 50000".to_string());
        }

        Ok(())
    }
}


#[derive(Deserialize, Serialize)]
pub struct UpdateSaldoRequest {
    #[serde(rename = "saldo_id")]
    pub saldo_id: i32,

    #[serde(rename = "user_id")]
    pub user_id: i32,

    #[serde(rename = "total_balance")]
    pub total_balance: i32,

    #[serde(rename = "withdraw_amount")]
    pub withdraw_amount: Option<i32>, 

    #[serde(rename = "withdraw_time")]
    pub withdraw_time: Option<NaiveDateTime>,  
}

impl UpdateSaldoRequest {
    pub fn validate(&self) -> Result<(), String> {
        if self.saldo_id <= 0 {
            return Err("Saldo ID must be greater than 0".to_string());
        }

        if self.user_id <= 0 {
            return Err("User ID must be greater than 0".to_string());
        }

        if self.total_balance < 0 {
            return Err("total balance must be greater than or equal to 50000".to_string());
        }

        if self.withdraw_amount.is_some() && self.withdraw_time.is_some() {
            return Err("Only one of withdraw_amount or withdraw_time can be provided".to_string());
        }

        if self.withdraw_amount.is_none() && self.withdraw_time.is_none() {
            return Err("Either withdraw_amount or withdraw_time must be provided".to_string());
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateSaldoBalance {
    pub total_balance: i32,
    pub user_id: i32,
    pub withdraw_amount: Option<i32>,
    pub withdraw_time: Option<NaiveDateTime>
}

impl UpdateSaldoBalance {
    pub fn validate(&self) -> Result<(), String> {
        if self.total_balance < 50000 {
            return Err("total balance must be greater than or equal to 50000".to_string());
        }

        if self.user_id <= 0 {
            return Err("User ID must be a positive integer".to_string());
        }

        Ok(())
    }
}