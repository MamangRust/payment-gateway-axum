use std::sync::Arc;

use async_trait::async_trait;
use mockall::automock;
use sea_orm::DbErr;

use crate::{
    domain::{
        request::{
            auth::RegisterRequest,
            user::{CreateUserRequest, UpdateUserRequest},
        },
        response::{user::UserResponse, ApiResponse, ErrorResponse},
    },
    entities::users,
};

pub type DynUserRepository = Arc<dyn UserRepositoryTrait + Send + Sync>;
pub type DynUserService = Arc<dyn UserServiceTrait + Send + Sync>;

#[automock]
#[async_trait]
pub trait UserRepositoryTrait {
    async fn find_all(&self) -> Result<Vec<users::Model>, DbErr>;
    async fn find_by_email_exists(&self, email: &str) -> Result<bool, DbErr>;
    async fn create_user(&self, input: &CreateUserRequest) -> Result<users::Model, DbErr>;
    async fn find_by_email(&self, email: &str) -> Result<Option<users::Model>, DbErr>;
    async fn find_by_id(&self, id: i32) -> Result<Option<users::Model>, DbErr>;
    async fn update_user(&self, input: &UpdateUserRequest) -> Result<users::Model, DbErr>;
    async fn delete_user(&self, id: i32) -> Result<(), DbErr>;
}

#[automock]
#[async_trait]
pub trait UserServiceTrait {
    async fn get_users(&self) -> Result<ApiResponse<Vec<UserResponse>>, ErrorResponse>;
    async fn find_by_id(&self, id: i32)
        -> Result<ApiResponse<Option<UserResponse>>, ErrorResponse>;
    async fn create_user(
        &self,
        input: &RegisterRequest,
    ) -> Result<ApiResponse<UserResponse>, ErrorResponse>;
    async fn update_user(
        &self,
        input: &UpdateUserRequest,
    ) -> Result<Option<ApiResponse<UserResponse>>, ErrorResponse>;
    async fn delete_user(&self, id: i32) -> Result<ApiResponse<()>, ErrorResponse>;
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use chrono::{DateTime, Utc};
//     use mockall::predicate::*;
//     use tokio;

//     #[tokio::test]
//     async fn test_find_all_users() {
//         let mut mock_repo = MockUserRepositoryTrait::new();

//         let mock_users = vec![
//             users::Model {
//                 user_id: 1,
//                 firstname: "John".to_string(),
//                 lastname: "Doe".to_string(),
//                 email: "johndoe@example.com".to_string(),
//                 password: "hashed_password".to_string(),
//                 noc_transfer: "12345".to_string(),
//                 created_at: Some(DateTime::<Utc>::from(Utc::now()).naive_utc()),
//                 updated_at: Some(DateTime::<Utc>::from(Utc::now()).naive_utc()),
//             },
//             users::Model {
//                 user_id: 1,
//                 firstname: "Jane".to_string(),
//                 lastname: "Smith".to_string(),
//                 email: "janesmith@example.com".to_string(),
//                 password: "hashed_password".to_string(),
//                 noc_transfer: "12345".to_string(),
//                 created_at: Some(DateTime::<Utc>::from(Utc::now()).naive_utc()),
//                 updated_at: Some(DateTime::<Utc>::from(Utc::now()).naive_utc()),
//             },
//         ];

//         mock_repo
//             .expect_find_all()
//             .returning(move || Ok(mock_users.clone()));

//         let result = mock_repo.find_all().await;

//         assert!(result.is_ok());
//         let users = result.unwrap();
//         assert_eq!(users.len(), 2);
//         assert_eq!(users[0].firstname, "John");
//         assert_eq!(users[1].firstname, "Jane");
//     }

//     #[tokio::test]
//     async fn test_find_by_email_exists() {
//         let mut mock_repo = MockUserRepositoryTrait::new();

//         let email = "johndoe@example.com";

//         mock_repo
//             .expect_find_by_email_exists()
//             .with(eq(email))
//             .returning(move |_| Ok(true));

//         let result = mock_repo.find_by_email_exists(email).await;

//         assert!(result.is_ok());
//         assert_eq!(result.unwrap(), true);
//     }

//     #[tokio::test]
//     async fn test_create_user() {
//         let mut mock_repo = MockUserRepositoryTrait::new();

//         let create_request = CreateUserRequest {
//             firstname: "John".to_string(),
//             lastname: "Doe".to_string(),
//             email: "johndoe@example.com".to_string(),
//             password: "secure_password".to_string(),
//             noc_transfer: None,
//             confirm_password: "secure_password".to_string(),
//         };

//         let created_user = users::Model {
//             user_id: 1,
//             firstname: create_request.firstname.clone(),
//             lastname: create_request.lastname.clone(),
//             email: create_request.email.clone(),
//             password: create_request.password.clone(),
//             noc_transfer: "12345".to_string(),
//             created_at: Some(DateTime::<Utc>::from(Utc::now()).naive_utc()),
//             updated_at: Some(DateTime::<Utc>::from(Utc::now()).naive_utc()),
//         };

//         mock_repo
//             .expect_create_user()
//             .with(eq(create_request.clone()))
//             .returning(move |_| Ok(created_user.clone()));

//         let result = mock_repo.create_user(&create_request).await;

//         assert!(result.is_ok());
//         let user = result.unwrap();
//         assert_eq!(user.firstname, "John");
//         assert_eq!(user.email, "johndoe@example.com");
//     }

//     #[tokio::test]
//     async fn test_find_by_email() {
//         let mut mock_repo = MockUserRepositoryTrait::new();

//         let email = "johndoe@example.com";
//         let mock_user = Some(users::Model {
//             user_id: 1,
//             firstname: "John".to_string(),
//             lastname: "Doe".to_string(),
//             email: email.to_string(),
//             password: "hashed_password".to_string(),
//             noc_transfer: "12345".to_string(),
//             created_at: Some(DateTime::<Utc>::from(Utc::now()).naive_utc()),
//             updated_at: Some(DateTime::<Utc>::from(Utc::now()).naive_utc()),
//         });

//         mock_repo
//             .expect_find_by_email()
//             .with(eq(email))
//             .returning(move |_| Ok(mock_user.clone()));

//         let result = mock_repo.find_by_email(email).await;

//         assert!(result.is_ok());
//         let user = result.unwrap();
//         assert!(user.is_some());
//         assert_eq!(user.unwrap().email, email);
//     }

//     #[tokio::test]
//     async fn test_update_user() {
//         let mut mock_repo = MockUserRepositoryTrait::new();

//         let update_request = UpdateUserRequest {
//             id: Some(1),
//             firstname: Some("John Updated".to_string()),
//             lastname: Some("Doe".to_string()),
//             email: Some("johnupdated@example.com".to_string()),
//             password: None,
//             confirm_password: None,
//         };

//         let updated_user = users::Model {
//             user_id: 1,
//             firstname: update_request.firstname.clone().unwrap(),
//             lastname: update_request.lastname.clone().unwrap(),
//             email: update_request.email.clone().unwrap(),
//             password: "hashed_password".to_string(),
//             noc_transfer: "12345".to_string(),
//             created_at: Some(DateTime::<Utc>::from(Utc::now()).naive_utc()),
//             updated_at: Some(DateTime::<Utc>::from(Utc::now()).naive_utc()),
//         };

//         mock_repo
//             .expect_update_user()
//             .with(eq(update_request.clone()))
//             .returning(move |_| Ok(updated_user.clone()));

//         let result = mock_repo.update_user(&update_request).await;

//         assert!(result.is_ok());
//         let user = result.unwrap();
//         assert_eq!(user.firstname, "John Updated");
//         assert_eq!(user.email, "johnupdated@example.com");
//     }

//     #[tokio::test]
//     async fn test_delete_user() {
//         let mut mock_repo = MockUserRepositoryTrait::new();

//         let user_id = 1;

//         mock_repo
//             .expect_delete_user()
//             .with(eq(user_id))
//             .returning(move |_| Ok(()));

//         let result = mock_repo.delete_user(user_id).await;

//         assert!(result.is_ok());
//     }
// }
