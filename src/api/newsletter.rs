use leptos::prelude::*;

#[server]
pub async fn subscribe_newsletter(email: String) -> Result<(), ServerFnError> {
    let email = crate::api::email_validation::parse_email(&email)?;
    insert(&email).await
}

#[cfg(feature = "ssr")]
async fn insert(email: &str) -> Result<(), ServerFnError> {
    let pool = expect_context::<sqlx::SqlitePool>();
    sqlx::query(
        "INSERT INTO newsletter_subscribers (email) VALUES (?1) \
         ON CONFLICT(email) DO NOTHING",
    )
    .bind(email)
    .execute(&pool)
    .await?;
    Ok(())
}
