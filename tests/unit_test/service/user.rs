use std::sync::Arc;

use chrono::{DateTime, Utc};
use example_payment_gateway_axum::{
    abstract_trait::{
        hashing::MockHashingTrait,
        user::{MockUserRepositoryTrait, UserRepositoryTrait, UserServiceTrait},
    },
    domain::request::{
        auth::RegisterRequest,
        user::{CreateUserRequest, UpdateUserRequest},
    },
    entities::users,
    services::user::UserService,
    utils::errors::AppError,
};
use mockall::predicate;
use sea_orm::DbErr;

#[tokio::test]
async fn test_get_users() {
    let mut mock_user_repo = MockUserRepositoryTrait::new();
    let mock_hashing = MockHashingTrait::new();

    let mock_users = vec![
        users::Model {
            user_id: 1,
            firstname: "John".to_string(),
            lastname: "Doe".to_string(),
            email: "john.doe@example.com".to_string(),
            password: "hashed_password".to_string(),
            noc_transfer: "12345".to_string(),
            created_at: Some(Utc::now().naive_utc()),
            updated_at: Some(Utc::now().naive_utc()),
        },
        users::Model {
            user_id: 2,
            firstname: "Jane".to_string(),
            lastname: "Doe".to_string(),
            email: "jane.doe@example.com".to_string(),
            password: "hashed_password".to_string(),
            noc_transfer: "67890".to_string(),
            created_at: Some(Utc::now().naive_utc()),
            updated_at: Some(Utc::now().naive_utc()),
        },
    ];

    mock_user_repo
        .expect_find_all()
        .return_once(move || Ok(mock_users.clone()));

    let service = UserService::new(Arc::new(mock_user_repo), Arc::new(mock_hashing));

    let result = service.get_users().await;

    if let Err(ref e) = result {
        println!("Error: {:?}", e);
    }

    assert!(result.is_ok());

    let response = result.unwrap();

    assert_eq!(response.status, "success");
    assert_eq!(response.message, "Users retrieved successfully");

    let data = response.data;

    assert_eq!(data.len(), 2);

    assert_eq!(data[0].id, 1);
    assert_eq!(data[0].firstname, "John");
    assert_eq!(data[0].lastname, "Doe");
    assert_eq!(data[0].email, "john.doe@example.com");

    assert_eq!(data[1].id, 2);
    assert_eq!(data[1].firstname, "Jane");
    assert_eq!(data[1].lastname, "Doe");
    assert_eq!(data[1].email, "jane.doe@example.com");
}

#[tokio::test]
async fn test_find_by_id_success() {
    let mut mock_user_repo = MockUserRepositoryTrait::new();
    let mock_hashing = MockHashingTrait::new();

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

    mock_user_repo
        .expect_find_by_id()
        .with(predicate::eq(user_id))
        .return_once(move |_| Ok(Some(mock_user.clone())));

    let service = UserService::new(Arc::new(mock_user_repo), Arc::new(mock_hashing));

    let result = service.find_by_id(user_id).await;

    if let Err(ref e) = result {
        println!("Error: {:?}", e);
    }

    assert!(result.is_ok());

    let response = result.unwrap();

    assert_eq!(response.status, "success");
    assert_eq!(response.message, "User retrieved successfully");

    let data = response.data.unwrap();

    assert_eq!(data.id, user_id);
    assert_eq!(data.firstname, "John");
    assert_eq!(data.lastname, "Doe");
    assert_eq!(data.email, "john.doe@example.com");
}

#[tokio::test]
async fn test_find_by_id_user_not_found() {
    let mut mock_user_repo = MockUserRepositoryTrait::new();
    let mock_hashing = MockHashingTrait::new();

    let user_id = 1;

    mock_user_repo
        .expect_find_by_id()
        .with(predicate::eq(user_id))
        .return_once(move |_| Ok(None));

    let service = UserService::new(Arc::new(mock_user_repo), Arc::new(mock_hashing));

    let result = service.find_by_id(user_id).await;

    if let Ok(ref response) = result {
        println!("Unexpected success: {:?}", response);
    }

    assert!(result.is_err());

    let error = result.unwrap_err();

    assert_eq!(error.status, "error");
    assert_eq!(error.message, format!("User with id {} not found", user_id));
}

#[tokio::test]
async fn test_create_user_success() {
    let mut mock_user_repo = MockUserRepositoryTrait::new();
    let mut mock_hashing = MockHashingTrait::new();

    let email = "john.doe@example.com".to_string();
    let email_clone = email.clone();

    let firstname = "John".to_string();
    let lastname = "Doe".to_string();
    let password = "password123".to_string();
    let hashed_password = "hashed_password123".to_string();
    let _noc_transfer = "12345".to_string();

    mock_user_repo
        .expect_find_by_email_exists()
        .with(predicate::eq(email.clone()))
        .return_once(move |_| Ok(false));

    mock_hashing
        .expect_hash_password()
        .with(predicate::eq(password.clone()))
        .return_once(move |_| Box::pin(async move { Ok(hashed_password.clone()) }));

    mock_user_repo
        .expect_create_user()
        .withf(move |req| req.email == email_clone)
        .return_once(move |req| {
            Ok(users::Model {
                user_id: 1,
                firstname: req.firstname.clone(),
                lastname: req.lastname.clone(),
                email: req.email.clone(),
                password: req.password.clone(),
                noc_transfer: req.noc_transfer.clone().unwrap_or_default(),
                created_at: Some(Utc::now().naive_utc()),
                updated_at: Some(Utc::now().naive_utc()),
            })
        });

    let service = UserService::new(Arc::new(mock_user_repo), Arc::new(mock_hashing));

    let input = RegisterRequest {
        firstname: firstname.clone(),
        lastname: lastname.clone(),
        email: email.clone(),
        password: password.clone(),
        confirm_password: password.clone(),
    };

    let result = service.create_user(&input).await;

    if let Err(ref e) = result {
        println!("Error: {:?}", e);
    }

    assert!(result.is_ok());

    let response = result.unwrap();

    assert_eq!(response.status, "success");
    assert_eq!(response.message, "User Create successfully");

    let data = response.data;
    assert_eq!(data.email, email);
    assert_eq!(data.firstname, firstname);
    assert_eq!(data.lastname, lastname);
}

#[tokio::test]
async fn test_create_user_email_exists() {
    let mut mock_user_repo = MockUserRepositoryTrait::new();
    let mock_hashing = MockHashingTrait::new();

    let email = "john.doe@example.com".to_string();

    mock_user_repo
        .expect_find_by_email_exists()
        .with(predicate::eq(email.clone()))
        .return_once(move |_| Ok(true));

    let service = UserService::new(Arc::new(mock_user_repo), Arc::new(mock_hashing));

    let input = RegisterRequest {
        firstname: "John".to_string(),
        lastname: "Doe".to_string(),
        email: email.clone(),
        password: "password123".to_string(),
        confirm_password: "password123".to_string(),
    };

    let result = service.create_user(&input).await;

    if let Ok(ref response) = result {
        println!("Unexpected success: {:?}", response);
    }

    assert!(result.is_err());

    let error = result.unwrap_err();

    assert_eq!(error.status, "error");
    assert_eq!(error.message, "Email already exists");
}

#[tokio::test]
async fn test_create_user_validation_error() {
    let mut mock_user_repo = MockUserRepositoryTrait::new();
    let mock_hashing = MockHashingTrait::new();

    let email = "john.doe@example.com".to_string();

    mock_user_repo
        .expect_find_by_email_exists()
        .with(predicate::eq(email.clone()))
        .return_once(move |_| Ok(false));

    let input = RegisterRequest {
        firstname: "".to_string(),
        lastname: "".to_string(),
        email: email.clone(),
        password: "password123".to_string(),
        confirm_password: "password123".to_string(),
    };

    let create_user_input = CreateUserRequest {
        firstname: input.firstname.clone(),
        lastname: input.lastname.clone(),
        email: input.email.clone(),
        password: input.password.clone(),
        noc_transfer: None,
        confirm_password: input.confirm_password.clone(),
    };

    mock_user_repo
        .expect_create_user()
        .with(predicate::eq(create_user_input))
        .return_once(move |_| {
            Err(DbErr::Custom(
                AppError::NotFound("User not found".to_string()).to_string(),
            ))
        });

    let service = UserService::new(Arc::new(mock_user_repo), Arc::new(mock_hashing));

    let result = service.create_user(&input).await;

    assert!(result.is_err());

    let error = result.unwrap_err();

    println!("Error: {:?}", error);

    assert_eq!(error.status, "Error Validation");
    assert!(error.message.contains("First name is required"));
}

#[tokio::test]
async fn test_update_user_success() {
    let mut mock_user_repo = MockUserRepositoryTrait::new();
    let mock_hashing = MockHashingTrait::new();

    let user_id = 1;
    let input = UpdateUserRequest {
        id: Some(user_id),
        firstname: Some("Updated Firstname".to_string()),
        lastname: Some("Updated Lastname".to_string()),
        email: Some("updated.email@example.com".to_string()),
        password: Some("newpassword".to_string()),
        confirm_password: Some("newpassword".to_string()),
    };

    let input_email = input.email.clone().unwrap();
    let input_firstname = input.firstname.clone().unwrap();
    let input_lastname = input.lastname.clone().unwrap();
    let input_password = input.password.clone().unwrap();

    mock_user_repo
        .expect_update_user()
        .with(predicate::eq(input.clone()))
        .return_once(move |_| {
            Ok(users::Model {
                user_id,
                firstname: input_firstname,
                lastname: input_lastname,
                email: input_email,
                password: input_password,
                noc_transfer: "12345".to_string(),
                created_at: Some(Utc::now().naive_utc()),
                updated_at: Some(Utc::now().naive_utc()),
            })
        });

    let service = UserService::new(Arc::new(mock_user_repo), Arc::new(mock_hashing));

    let result = service.update_user(&input.clone()).await;

    assert!(result.is_ok());

    let response = result.unwrap().unwrap();
    assert_eq!(response.status, "success");
    assert_eq!(response.message, "User updated successfully");

    let data = response.data;
    assert_eq!(data.firstname, "Updated Firstname");
    assert_eq!(data.lastname, "Updated Lastname");
    assert_eq!(data.email, "updated.email@example.com");
}

#[tokio::test]
async fn test_update_user_validation_error() {
    let mut mock_user_repo = MockUserRepositoryTrait::new();
    let mock_hashing = MockHashingTrait::new();

    let email = "john.doe@example.com".to_string();

    mock_user_repo
        .expect_find_by_email_exists()
        .with(predicate::eq(email.clone()))
        .return_once(move |_| Ok(true));

    let user_id = 1;
    let input = UpdateUserRequest {
        id: Some(user_id),
        firstname: Some("".to_string()),
        lastname: Some("Updated Lastname".to_string()),
        email: Some(email.clone()),
        password: Some("newpassword".to_string()),
        confirm_password: Some("newpassword".to_string()),
    };

    mock_user_repo
        .expect_update_user()
        .with(predicate::eq(input.clone()))
        .return_once(move |_| {
            Err(DbErr::Custom(
                AppError::NotFound("User not found".to_string()).to_string(),
            ))
        });

    let service = UserService::new(Arc::new(mock_user_repo), Arc::new(mock_hashing));

    let result = service.update_user(&input).await;

    assert!(result.is_err());

    let error = result.unwrap_err();

    println!("Error: {}", error);

    assert_eq!(error.status, "error");
    assert!(error.message.contains("Database error occurred"));
}

#[tokio::test]
async fn test_delete_user_success() {
    let mut mock_user_repo = MockUserRepositoryTrait::new();

    let user_id = 1;

    mock_user_repo
        .expect_delete_user()
        .with(predicate::eq(user_id))
        .return_once(|_| Ok(()));

    let service = UserService::new(Arc::new(mock_user_repo), Arc::new(MockHashingTrait::new()));

    let result = service.delete_user(user_id).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.status, "success");
    assert_eq!(response.message, "User deleted successfully");
    assert_eq!(response.data, ());
}

#[tokio::test]
async fn test_delete_user_error() {
    let mut mock_user_repo = MockUserRepositoryTrait::new();

    let user_id = 1;

    mock_user_repo
        .expect_delete_user()
        .with(predicate::eq(user_id))
        .return_once(|_| Err(DbErr::Custom("User not found".to_string())));

    let service = UserService::new(Arc::new(mock_user_repo), Arc::new(MockHashingTrait::new()));

    let result = service.delete_user(user_id).await;

    assert!(result.is_err());
    let error = result.unwrap_err();

    println!("Error: {}", error);

    assert_eq!(error.status, "error");
    assert!(error.message.contains("Database error occurred"));
}
