use std::net::TcpListener;
use fake::Fake;
use fake::faker::internet::en::{Password, Username, SafeEmail};
use fake::faker::name::en::Name;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
use uuid::Uuid;
use insta::app::app;
use insta::utils::password;
use insta::db;
use insta::configuration::Settings;

#[derive(Clone)]
pub struct MyTestServer {
    pub address: String,
    pub config: Settings,
    pub db: DatabaseConnection,
}

fn set_testing_env() {
    std::env::set_var("APP_APPLICATION__RUST_ENV", "testing");
}

pub async fn start_test_server() -> MyTestServer {
    set_testing_env();

    let host = String::from("127.0.0.1");
    let listener =
        TcpListener::bind(format!("{}:0", &host.to_owned())).expect("Failed to bind TCP Listener");

    let port = listener.local_addr().unwrap().port();
    let address = format!("http://{}:{}", host, port);

    let config = Settings::get_configuration();

    let server = app(listener, config.clone())
        .await
        .expect("Failed to get the server: ");

    actix_web::rt::spawn(server);

    let db = db::connect_db(&config)
        .await
        .expect("Failed to connect to the database");

    MyTestServer { address, config, db }
}

pub async fn create_random_user(db: &DatabaseConnection) -> (entity::users::Model, String) {
    let random_password: String = Password(5..10).fake();

    // Create a user
    let model = entity::users::ActiveModel {
        id: Set(Uuid::new_v4().into()),
        email: Set(SafeEmail().fake::<String>()),
        name: Set(Name().fake()),
        bio: Set(None),
        picture_url: Set("".to_owned()),
        username: Set(Username().fake()),
        password: Set(password::hash_password(random_password.as_str()).expect("password hashed")),
        ..Default::default()
    }.insert(db).await.expect("Failed to insert user");

    (model, random_password)
}

pub async fn delete_user(db: &DatabaseConnection, user_id: &[u8]) {
    entity::users::Entity::delete_by_id(Uuid::from_slice(user_id).unwrap())
        .exec(db)
        .await
        .expect("Failed to delete user");
}

pub async fn parse_response_body<T>(resp: reqwest::Response) -> T
    where
        T: serde::de::DeserializeOwned,
{
    let resp_text = resp.text().await.unwrap();
    serde_json::from_str(&resp_text).unwrap()
}