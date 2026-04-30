use leptos::prelude::*;

#[server]
pub async fn subscribe_newsletter(email: String) -> Result<(), ServerFnError> {
    let email = parse_email(&email)?;
    insert(&email).await
}

#[cfg_attr(not(feature = "ssr"), allow(dead_code))]
fn parse_email(input: &str) -> Result<String, ServerFnError> {
    let trimmed = input.trim().to_lowercase();
    if !trimmed.contains('@') || trimmed.len() < 5 {
        return Err(ServerFnError::new("Indirizzo email non valido."));
    }
    Ok(trimmed)
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

#[cfg(test)]
mod tests {
    use super::parse_email;

    #[test]
    fn rejects_missing_at() {
        assert!(parse_email("nope").is_err());
    }

    #[test]
    fn rejects_too_short() {
        assert!(parse_email("a@b").is_err());
    }

    #[test]
    fn trims_and_lowercases() {
        assert_eq!(
            parse_email("  Me@Example.com  ").unwrap(),
            "me@example.com"
        );
    }

    #[test]
    fn accepts_minimal_valid() {
        assert_eq!(parse_email("a@b.c").unwrap(), "a@b.c");
    }
}
