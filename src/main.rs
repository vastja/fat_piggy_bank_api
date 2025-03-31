use axum::{extract::Query, routing::get, Json, Router};
use chrono::{DateTime, Utc};
use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/api/expenses", get(expenses));

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[derive(Deserialize)]
struct ExpensesParams {
    group: bool,
}

async fn expenses(Query(params): Query<ExpensesParams>) -> Json<Vec<Expense>> {
    let connection = Connection::open("../fat_piggy_bank_importer/fat_piggy_bank.db")
        .expect("Database connection failed.");

    let mut select = match params.group {
        true => connection
            .prepare("SELECT id, MIN(date), tag, SUM(amount) FROM expenses GROUP BY tag")
            .expect("Retrieving expenses failed."),
        false => connection
            .prepare("SELECT * FROM expenses")
            .expect("Retrieving expenses failed."),
    };

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
