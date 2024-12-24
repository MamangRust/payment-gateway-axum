use crate::utils::errors::AppError;
use async_trait::async_trait;
use bcrypt::BcryptError;
use mockall::automock;
use std::sync::Arc;

#[async_trait]
#[automock]
pub trait HashingTrait: Send + Sync {
    async fn hash_password(&self, password: &str) -> Result<String, BcryptError>;
    async fn compare_password(&self, hashed_password: &str, password: &str)
        -> Result<(), AppError>;
}

pub type DynHashing = Arc<dyn HashingTrait + Send + Sync>;

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use mockall::predicate::*;

//     #[tokio::test]
//     async fn test_hash_password() {
//         let mut mock_hashing = MockHashingTrait::new();
//         let plain_password = "secure_password";

//         mock_hashing
//             .expect_hash_password()
//             .with(eq(plain_password))
//             .times(1)
//             .returning(|_| {
//                 Box::pin(async {
//                     Ok("$2b$04$abcdefghijk1234567890qrstuvwxyzABCDabcd123456".to_string())
//                 })
//             });

//         let result = mock_hashing.hash_password(plain_password).await;
//         assert!(result.is_ok());
//         assert_eq!(
//             result.unwrap(),
//             "$2b$04$abcdefghijk1234567890qrstuvwxyzABCDabcd123456"
//         );
//     }

//     #[tokio::test]
//     async fn test_compare_password_match() {
//         let mut mock_hashing = MockHashingTrait::new();
//         let plain_password = "secure_password";
//         let hashed_password = "$2b$04$abcdefghijk1234567890qrstuvwxyzABCDabcd123456";

//         mock_hashing
//             .expect_compare_password()
//             .with(eq(hashed_password), eq(plain_password))
//             .times(1)
//             .returning(|_, _| Box::pin(async { Ok(()) }));

//         let result = mock_hashing
//             .compare_password(hashed_password, plain_password)
//             .await;
//         assert!(result.is_ok());
//     }

//     #[tokio::test]
//     async fn test_compare_password_mismatch() {
//         let mut mock_hashing = MockHashingTrait::new();
//         let plain_password = "secure_password";
//         let hashed_password = "$2b$04$abcdefghijk1234567890qrstuvwxyzABCDabcd123456";

//         mock_hashing
//             .expect_compare_password()
//             .with(eq(hashed_password), eq(plain_password))
//             .times(1)
//             .returning(|_, _| {
//                 Box::pin(async {
//                     Err(AppError::HashingError(BcryptError::from(
//                         std::io::Error::new(std::io::ErrorKind::Other, "Passwords do not match."),
//                     )))
//                 })
//             });

//         let result = mock_hashing
//             .compare_password(hashed_password, plain_password)
//             .await;

//         match result {
//             Err(AppError::HashingError(_)) => assert!(true),
//             _ => panic!("Expected AppError::HashingError"),
//         }
//     }
// }
