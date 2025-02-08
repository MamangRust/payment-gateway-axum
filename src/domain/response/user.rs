use crate::entities::users;
use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct UserResponse {
    pub id: i32,
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub noc_transfer: String,
    #[schema(format = "date-time")]
    pub created_at: Option<DateTime<Utc>>,

    #[schema(format = "date-time")]
    pub updated_at: Option<DateTime<Utc>>,
}

impl From<users::Model> for UserResponse {
    fn from(value: users::Model) -> Self {
        UserResponse {
            id: value.user_id,
            firstname: value.firstname,
            lastname: value.lastname,
            email: value.email,
            noc_transfer: value.noc_transfer,
            created_at: value.created_at.map(|dt| Utc.from_utc_datetime(&dt)),
            updated_at: value.updated_at.map(|dt| Utc.from_utc_datetime(&dt)),
        }
    }
}
