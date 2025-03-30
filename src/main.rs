use axum::{routing::get, Json, Router};
use chrono::{Date, DateTime, Utc};
use rusqlite::{Connection, Result};
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
    let connection = Connection::open("../fat_piggy_bank_importer/fat_piggy_bank.db")
        .expect("Database connection failed.");

    let mut select = connection
        .prepare("SELECT * FROM expenses")
        .expect("Retrieving expenses failed.");

    let expenses: Result<Vec<Expense>> = select
        .query_map([], |row| {
            Ok(Expense {
                id: row.get(0)?,
                date: row.get(1)?,
                tag: row.get(2)?,
                amount: row.get(3)?,
            })
        })
        .unwrap()
        .collect();

    Json(expenses.unwrap())
}

#[derive(Serialize)]
struct Expense {
    id: u32,
    date: DateTime<Utc>,
    tag: String,
    amount: i32,
}
