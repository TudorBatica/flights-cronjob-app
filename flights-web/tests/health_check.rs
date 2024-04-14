use flights_web::configuration::get_configuration;
use sqlx::{PgPool, Pool, Postgres};
use std::net::TcpListener;

struct TestApp {
    addr: String,
    db_pool: Pool<Postgres>,
}

async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = format!("http://127.0.0.1:{}", listener.local_addr().unwrap().port());
    let config = get_configuration();
    let pool = PgPool::connect(&config.database_url).await.unwrap();
    let server = flights_web::startup::run(listener, pool.clone()).unwrap();
    let _ = tokio::spawn(server);

    TestApp {
        addr,
        db_pool: pool,
    }
}

#[tokio::test]
async fn health_check_works() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/health", app.addr))
        .send()
        .await
        .unwrap();

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn posting_new_route_monitor_returns_200_for_valid_form_data() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let request_body = r#"[
          {
            "from_airport_code": "OTP",
            "to_country_code": "IT",
            "budget": 50,
            "trip_type": 1
          },
          {
            "from_airport_code": "OTP",
            "to_country_code": "GR",
            "budget": 55,
            "trip_type": 2
          }
        ]
    "#;
    let response = client
        .post(&format!("{}/monitor", &app.addr))
        .header("Content-Type", "application/json")
        .body(request_body)
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);

    let saved = sqlx::query!("SELECT * FROM monitored_routes")
        .fetch_all(&app.db_pool)
        .await
        .unwrap();

    assert_eq!(saved.len(), 2);
}
