use chrono::{Duration, Utc};
use reqwest::{Client, StatusCode};
use insta::error::HttpResponseError;
use fake::Fake;
use fake::faker::internet::en::{Password, SafeEmail, Username};
use fake::faker::name::en::{Name, Title};
use jsonwebtoken::{encode, EncodingKey};
use sea_orm::{ColumnTrait, QueryFilter, EntityTrait};
use insta::auth::JwtTokenPayload;
use insta::db::connect_db;
use crate::utils::{create_random_user, delete_user, parse_response_body};

mod utils;

// ---- SIGN UP UNIT TESTS ----

#[actix_web::test]
async fn signup_should_required_signup_data() {
    let app = utils::start_test_server().await;
    let client = Client::new();

    let resp = client.post(&format!("{}/api/v1/auth/signup", &app.address))
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({}))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status().as_u16(), 400);

    let response_body = resp.text().await.unwrap();
    let response_body: HttpResponseError = serde_json::from_str(&response_body).unwrap();

    assert_eq!(response_body.code, Some(StatusCode::BAD_REQUEST.as_u16()));

    let errors = response_body.errors;

    let email_error = errors.iter().any(|e| e.field == Some("email".to_owned()));
    let full_name_error = errors.iter().any(|e| e.field == Some("fullName".to_owned()));
    let username_error = errors.iter().any(|e| e.field == Some("username".to_owned()));
    let bio_error = errors.iter().any(|e| e.field == Some("bio".to_owned()));
    let password_error = errors.iter().any(|e| e.field == Some("password".to_owned()));
    let c_password_error = errors.iter().any(|e| e.field == Some("confirmPassword".to_owned()));

    assert!(email_error);
    assert!(full_name_error);
    assert!(username_error);
    assert!(password_error);
    assert!(c_password_error);

    // Because bio is not required
    assert!(!bio_error);
}

#[actix_web::test]
async fn signup_password_and_c_password_should_match() {
    let app = utils::start_test_server().await;
    let client = Client::new();

    let resp = client.post(&format!("{}/api/v1/auth/signup", &app.address))
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({
            "email": SafeEmail().fake::<String>(),
            "fullName": Name().fake::<String>(),
            "username": Username().fake::<String>(),
            "password": Password(5..10).fake::<String>(),
            "confirmPassword": Password(5..10).fake::<String>()
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status().as_u16(), 400);

    let response_body = resp.text().await.unwrap();
    let response_body: HttpResponseError = serde_json::from_str(&response_body).unwrap();

    assert_eq!(response_body.code, Some(StatusCode::BAD_REQUEST.as_u16()));

    let errors = response_body.errors;
    assert_eq!(errors.len(), 1);
    let c_password_error = errors.iter().any(|e| e.field == Some("confirmPassword".to_owned()));

    assert!(c_password_error);
}

#[actix_web::test]
async fn signup_should_not_allow_username_with_non_allowed_character() {
    let app = utils::start_test_server().await;
    let client = Client::new();

    let password: String = Password(5..10).fake();
    let username: String = format!("{}@&_hello", Username().fake::<String>());

    let resp = client.post(&format!("{}/api/v1/auth/signup", &app.address))
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({
            "email": SafeEmail().fake::<String>(),
            "fullName": Name().fake::<String>(),
            "username": username,
            "password": &password,
            "confirmPassword": &password
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status().as_u16(), 400);

    let response_body = resp.text().await.unwrap();
    let response_body: HttpResponseError = serde_json::from_str(&response_body).unwrap();

    assert_eq!(response_body.code, Some(StatusCode::BAD_REQUEST.as_u16()));

    let errors = response_body.errors;
    assert_eq!(errors.len(), 1);
    let username_error = errors.iter().find(|e| e.field == Some("username".to_owned()));

    assert!(username_error.is_some());
    assert_eq!(username_error.unwrap().message, Some("Username should not contains non-allowed character".to_owned()));
}

#[actix_web::test]
async fn signup_should_success() {
    let app = utils::start_test_server().await;
    let client = Client::new();

    let user_email: String = SafeEmail().fake();

    let password = Password(5..10).fake::<String>();

    let resp = client.post(&format!("{}/api/v1/auth/signup", &app.address))
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({
            "email": user_email,
            "fullName": Name().fake::<String>(),
            "username": Username().fake::<String>(),
            "password": &password,
            "confirmPassword": &password,
            "bio": Title().fake::<String>()
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status().as_u16(), StatusCode::CREATED.as_u16());

    let response_body = resp.text().await.unwrap();
    let response_body: serde_json::Value = serde_json::from_str(&response_body).unwrap();

    assert_eq!(response_body.get("code").unwrap().as_u64(), Some(StatusCode::CREATED.as_u16() as u64));

    let user = entity::users::Entity::find()
        .filter(entity::users::Column::Email.eq(user_email))
        .one(&app.db)
        .await
        .expect("Failed to find user");

    assert!(user.is_some());

    let user = user.unwrap();
    utils::delete_user(&app.db, &user.id).await;

    assert_ne!(user.password, password);
    assert!(user.password.contains("argon2"));
}

#[actix_web::test]
async fn signup_should_not_allowed_duplicate_email() {
    let app = utils::start_test_server().await;
    let db = connect_db(&app.config).await.expect("Failed to connect to the database");

    // Create a user
    let (created_user, password) = create_random_user(&app.db).await;

    let client = Client::new();

    let resp = client.post(&format!("{}/api/v1/auth/signup", &app.address))
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({
            "email": &created_user.email,
            "fullName": Name().fake::<String>(),
            "username": Username().fake::<String>(),
            "password": &password,
            "confirmPassword": &password,
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status().as_u16(), 400);

    let response_body = resp.text().await.unwrap();
    let response_body: HttpResponseError = serde_json::from_str(&response_body).unwrap();

    assert_eq!(response_body.code, Some(StatusCode::BAD_REQUEST.as_u16()));

    let resp_error = response_body.errors.first().unwrap();
    println!("{:?}", resp_error);

    utils::delete_user(&db, &created_user.id).await;

    assert_eq!(resp_error.error, Some("This email is already taken".to_owned()))
}

// ---- END OF SIGN UP UNIT TESTS ----

// ---- SIGN IN UNIT TESTS ----

#[actix_web::test]
async fn signin_should_required_signin_data() {
    let app = utils::start_test_server().await;
    let client = Client::new();

    let resp = client.post(&format!("{}/api/v1/auth", &app.address))
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({}))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status().as_u16(), 400);

    let response_body: HttpResponseError = parse_response_body(resp).await;
    assert_eq!(response_body.code, Some(StatusCode::BAD_REQUEST.as_u16()));

    let errors = response_body.errors;

    assert_eq!(errors.len(), 2);

    let email_username_error = errors.iter().any(|e| e.field == Some("emailUsername".to_owned()));
    let password_error = errors.iter().any(|e| e.field == Some("password".to_owned()));

    assert!(email_username_error);
    assert!(password_error);
}

#[actix_web::test]
async fn signin_should_not_allow_wrong_credentials() {
    let app = utils::start_test_server().await;
    let client = Client::new();

    let (created_user, _p) = create_random_user(&app.db).await;

    let resp = client.post(&format!("{}/api/v1/auth", &app.address))
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({
            "emailUsername": &created_user.email,
            "password": Password(5..10).fake::<String>(),
        }))
        .send()
        .await
        .unwrap();

    delete_user(&app.db, &created_user.id).await;

    assert_eq!(resp.status().as_u16(), 400);

    let response_body: HttpResponseError = parse_response_body(resp).await;
    assert_eq!(response_body.code, Some(StatusCode::BAD_REQUEST.as_u16()));

    let errors = response_body.errors.first().unwrap();

    assert_eq!(errors.error, Some("Your email/username and password are wrong!".to_owned()));
}

#[actix_web::test]
async fn signin_email_password_should_valid() {
    let app = utils::start_test_server().await;
    let client = Client::new();

    let (created_user, password) = create_random_user(&app.db).await;

    let resp = client.post(&format!("{}/api/v1/auth", &app.address))
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({
            "emailUsername": &created_user.email,
            "password": &password,
        }))
        .send()
        .await
        .unwrap();

    delete_user(&app.db, &created_user.id).await;

    assert_eq!(resp.status().as_u16(), 200);

    let response_body: serde_json::Value = parse_response_body(resp).await;
    assert_eq!(response_body["code"].as_u64(), Some(StatusCode::OK.as_u16() as u64));
}

#[actix_web::test]
async fn signin_username_password_should_valid() {
    let app = utils::start_test_server().await;
    let client = Client::new();

    let (created_user, password) = create_random_user(&app.db).await;

    let resp = client.post(&format!("{}/api/v1/auth", &app.address))
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({
            "emailUsername": &created_user.username,
            "password": &password
        }))
        .send()
        .await
        .unwrap();

    delete_user(&app.db, &created_user.id).await;

    assert_eq!(resp.status().as_u16(), 200);

    let response_body: serde_json::Value = parse_response_body(resp).await;
    assert_eq!(response_body["code"].as_u64(), Some(StatusCode::OK.as_u16() as u64));
}

#[actix_web::test]
async fn signin_should_return_proper_data() {
    let app = utils::start_test_server().await;
    let client = Client::new();

    let (created_user, password) = create_random_user(&app.db).await;

    let resp = client.post(&format!("{}/api/v1/auth", &app.address))
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({
            "emailUsername": &created_user.email,
            "password": &password
        }))
        .send()
        .await
        .unwrap();

    delete_user(&app.db, &created_user.id).await;

    assert_eq!(resp.status().as_u16(), 200);

    let response_body: serde_json::Value = parse_response_body(resp).await;
    assert_eq!(response_body["code"].as_u64(), Some(StatusCode::OK.as_u16() as u64));

    assert!(response_body["data"].is_object());
    assert!(response_body["token"].is_string());
    assert!(response_body["refreshToken"].is_string());

    let data = response_body["data"].as_object().unwrap();

    // Deep down to data fields
    assert!(data.get("id").is_some());
    assert!(data.get("email").is_some());
    assert!(data.get("fullName").is_some());
    assert!(data.get("email").is_some());
    assert!(data.get("pictureUrl").is_some());
}

// ---- END OF SIGN IN UNIT TESTS ----

// ---- GET NEW TOKEN UNIT TESTS ----

#[actix_web::test]
async fn getnewtoken_should_required_refresh_token() {
    let app = utils::start_test_server().await;
    let client = Client::new();

    // Try to request a new token
    let resp = client.post(&format!("{}/api/v1/auth/token", &app.address))
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({}))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);

    let response_body: HttpResponseError = parse_response_body(resp).await;

    assert_eq!(response_body.errors.len(), 1);
    let errors = response_body.errors.iter().find(|e| e.field == Some("refreshToken".to_owned()));

    assert_eq!(errors.unwrap().message, Some("Please provide your refresh token to get a new access token".to_owned()));
}

#[actix_web::test]
async fn getnewtoken_should_success() {
    let app = utils::start_test_server().await;
    let client = Client::new();

    let (created_user, password) = create_random_user(&app.db).await;

    // Logged in the user
    let resp = client.post(&format!("{}/api/v1/auth", &app.address))
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({
            "emailUsername": &created_user.email,
            "password": &password
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::OK);

    let response_body: serde_json::Value = parse_response_body(resp).await;

    let resp_code = response_body["code"].as_u64();
    assert_eq!(resp_code, Some(StatusCode::OK.as_u16() as u64));

    let refresh_token = response_body["refreshToken"].as_str().expect("Token existed here!");

    // Try to request a new token
    let resp = client.post(&format!("{}/api/v1/auth/token", &app.address))
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({
            "refreshToken": refresh_token
        }))
        .send()
        .await
        .unwrap();

    delete_user(&app.db, &created_user.id).await;

    assert_eq!(resp.status(), StatusCode::OK);

    let response_body: serde_json::Value = parse_response_body(resp).await;

    let resp_code = response_body["code"].as_u64();
    assert_eq!(resp_code, Some(StatusCode::OK.as_u16() as u64));

    assert!(response_body["token"].as_str().is_some());
}

// ---- END OF GET NEW TOKEN UNIT TESTS ----

// ---- GET ME UNIT TESTS ----

#[actix_web::test]
async fn getme_should_unauthorized_invalid_jwt() {
    let app = utils::start_test_server().await;
    let client = Client::new();

    let token_payload = JwtTokenPayload {
        aud: JwtTokenPayload::get_audience(),
        // Expired immediately
        exp: (Utc::now() + Duration::microseconds(0)).timestamp(),
        id: uuid::Uuid::new_v4().to_string(),
        username: "random_name".to_string(),
        picture_url: "".to_string(),
        full_name: "".to_string(),
        email: "".to_string(),
    };

    // Sign with invalid secret

    let encoding_key = EncodingKey::from_secret("HELLO WORLD".as_bytes());
    let invalid_token = encode(&jsonwebtoken::Header::default(), &token_payload, &encoding_key).expect("should encode jwt");

    let resp = client.get(&format!("{}/api/v1/auth/me", &app.address))
        .header("Accept", "application/json")
        .bearer_auth(&invalid_token)
        .send()
        .await
        .expect("should return resp");

    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);

    let response_body: HttpResponseError = parse_response_body(resp).await;

    assert_eq!(response_body.code, Some(StatusCode::BAD_REQUEST.as_u16()));

    let error = response_body.errors.first().unwrap();
    assert_eq!(error.error, Some("Invalid JWT token".to_owned()));
}

#[actix_web::test]
async fn getme_should_success() {
    let app = utils::start_test_server().await;
    let client = Client::new();

    let (created_user, password) = create_random_user(&app.db).await;

    // Logged in the user
    let resp = client.post(&format!("{}/api/v1/auth", &app.address))
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({
            "emailUsername": &created_user.email,
            "password": &password
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::OK);

    let response_body: serde_json::Value = parse_response_body(resp).await;

    let resp_code = &response_body["code"].as_u64();
    assert_eq!(resp_code, &Some(StatusCode::OK.as_u16() as u64));

    let token = response_body["token"].as_str().unwrap();

    // Try to request a new token
    let resp = client.get(&format!("{}/api/v1/auth/me", &app.address))
        .header("Accept", "application/json")
        .bearer_auth(token)
        .send()
        .await
        .unwrap();

    delete_user(&app.db, &created_user.id).await;

    assert_eq!(resp.status(), StatusCode::OK);

    let response_body: serde_json::Value = parse_response_body(resp).await;

    let resp_code = &response_body["code"].as_u64();
    assert_eq!(resp_code, &Some(StatusCode::OK.as_u16() as u64));

    let resp_data = &response_body["data"];
    let resp_data = serde_json::from_value::<JwtTokenPayload>(resp_data.to_owned());

    assert!(resp_data.is_ok());
}

// ---- END OF GET ME UNIT TESTS ----