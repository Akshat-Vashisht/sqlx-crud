// use axum::{Router, routing::get};
use chrono::Local;
use dotenv::dotenv;
use sqlx::{postgres::PgPool, Row};
use std::env;

#[derive(Debug)]
struct Todo {
    title: String,
    created_at: String,
}

async fn create(todo: &Todo, pool: &PgPool) {
    let query = "INSERT INTO todos (title, created_at) VALUES ($1, CAST($2 as TIMESTAMP))";
    sqlx::query(query)
        .bind(&todo.title)
        .bind(&todo.created_at)
        .execute(pool)
        .await
        .expect("Failed to insert into DB");
}

async fn read(pool: &PgPool) {
    let query = "SELECT title, CAST(created_at as VARCHAR) from todos";
    let rows = sqlx::query(query)
        .fetch_all(pool)
        .await
        .expect("Failed to read");
    let todos: Vec<_> = rows
        .iter()
        .map(|row| Todo {
            title: row.get("title"),
            created_at: row.get("created_at"),
        })
        .collect();
    println!("{todos:#?}");
}

async fn update(id: i32, pool: &PgPool, title: String) {
    let query = "UPDATE todos SET title = $1 WHERE id = $2";

    sqlx::query(query)
        .bind(&title)
        .bind(&id)
        .execute(pool)
        .await
        .expect(&format!("Error updating value for book id: {id}"));
}

async fn delete(id: i32, pool: &PgPool) {
    let query = "DELETE FROM todos WHERE id = $1";

    sqlx::query(query)
        .bind(&id)
        .execute(pool)
        .await
        .expect(&format!("Error deleting book with id: {id}"));
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let url = env::var("DATABASE_URL").unwrap();
    let pool = PgPool::connect(&url).await.unwrap();

    // ADDING TWO TODOS (CREATE)
    let todo = Todo {
        title: "Learn SQLX".to_string(),
        created_at: Local::now().to_string(),
    };
    create(&todo, &pool).await;

    let todo = Todo {
        title: "Learn Diesel".to_string(),
        created_at: Local::now().to_string(),
    };
    create(&todo, &pool).await;

    // DISPLAYING TODOS (READ)
    read(&pool).await;

    // Updating todo from Learn Diesel to Learn Axum
    update(2, &pool, "Learn Axum".to_string()).await;

    // Displaying again
    read(&pool).await;

    // Deleting book with id 1
    delete(1, &pool).await;

    // Displaying again
    read(&pool).await;
}
