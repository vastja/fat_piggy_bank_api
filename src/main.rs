use axum::{routing::get, Json, Router};
use chrono::{DateTime, TimeZone, Utc};
use serde::Serialize;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/api", get(expenses));

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

async fn expenses() -> Json<Vec<Expense>> {
    let expenses = vec![Expense {
        id: 0,
        date: Utc.with_ymd_and_hms(2025, 5, 17, 0, 0, 0).unwrap(),
        tag: String::from("Groccery"),
        amount: 100,
    }];
    Json(expenses)
}

#[derive(Serialize)]
struct Expense {
    id: u32,
    date: DateTime<Utc>,
    tag: String,
    amount: i32,
}
