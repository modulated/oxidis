use actix_web::{web, HttpResponse};
use sqlx::SqliteConnection;
use std::sync::Arc;
use chrono::Utc;
use std::ops::Deref;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscriptions(form: web::Form<FormData>, connection: web::Data<Arc<SqliteConnection>>) -> Result<HttpResponse, HttpResponse> {
    let msg = format!("Name: {}, Email: {}", form.name, form.email);
    let timestamp = Utc::now().to_string();
    sqlx::query!(
        r#"INSERT INTO subscriptions (email, name, subscribed_at)
        VALUES (?1, ?2, ?3)
        "#,
        form.email, form.name, timestamp
    )
        .get_one(connection.get_ref().deref())
        .await
        .map_err(|e| {
            eprintln!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        })?;


    Ok(HttpResponse::Ok().finish())
}