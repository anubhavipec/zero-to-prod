use std::fmt::format;
use std::net::TcpListener;
use once_cell::sync::Lazy;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;
use zero_to_prod::configuration::{DatabaseSettings, get_configuration};
use zero_to_prod::telemetry::{get_subscriber, init_subscriber};

#[tokio::test]
async fn health_check_works() {

    //Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    // Act
    let response = client.get(&format!("{}/health_check",&app.address))
        .send()
        .await
        .expect("Failed to execute request");

    //Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0),response.content_length());

}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {

    let app = spawn_app().await;
    let client = reqwest::Client::new();

    //Act

    let body = "name=Anubhav%20Gupta&email=dummy%40gmail.com";

    let response = client
        .post(&format!("{}/subscriptions",&app.address))
        .header("Content-Type","application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    //Assert

    assert_eq!(200,response.status().as_u16());

    let saved = sqlx::query!("Select email,name from subscriptions",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription");
    let saved = sqlx::query!("SELECT email, name FROM subscriptions",) .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription.");
    assert_eq!(saved.email, "dummy@gmail.com");
    assert_eq!(saved.name, "Anubhav Gupta");
}

#[tokio::test]
async  fn subscribe_returns_a_400_when_data_is_missing() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email")
    ];
    //Act

    for (invalid_body,error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions",&app.address))
            .header("Content-Type","application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request");

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API didnt fail with 400 Bad Request for payload {}.",error_message
        );

    }
}

static TRACING: Lazy<()> = Lazy::new(|| {
    let subscriber = get_subscriber("test".into(),"debug".into());
    init_subscriber(subscriber);
});

pub struct TestApp{
    pub address: String,
    pub db_pool: PgPool,
}

async fn spawn_app() -> TestApp  {

    Lazy::force(&TRACING);
    let subscriber = get_subscriber("test".into(),"debug".into());
    init_subscriber(subscriber);

    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind to random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}",port);

    let mut configuration = get_configuration().expect("Failed to read configuration");
    configuration.database.database_name = Uuid::new_v4().to_string();
    let connection_pool = configure_database(&configuration.database).await;

    let server = zero_to_prod::startup::run(listener,connection_pool.clone()).expect("Failed to bind address");

    let _ = tokio::spawn(server);
    TestApp{
        address,
        db_pool: connection_pool,
    }

}

pub async fn configure_database(config : &DatabaseSettings) -> PgPool {

    let mut connection  = PgConnection::connect(&config.connection_string_without_db())
        .await
        .expect("Failed to connect to postgres");

    connection.execute(format!(r#"Create database "{}";"#,config.database_name).as_str())
        .await
        .expect("Failed to create database");

    //Migrate Database

    let connection_pool = PgPool::connect(&config.connection_string())
        .await
        .expect("Failed to connect to postgres");

    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");
    connection_pool
}