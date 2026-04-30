use crate::api::email_validation::parse_email;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct InquiryInput {
    pub product_slug: Option<String>,
    pub name: String,
    pub email: String,
    pub message: String,
}

#[server]
pub async fn submit_inquiry(
    product_slug: Option<String>,
    name: String,
    email: String,
    message: String,
) -> Result<(), ServerFnError> {
    let validated = validate_inquiry(InquiryInput {
        product_slug,
        name,
        email,
        message,
    })?;
    insert(&validated).await
}

#[cfg_attr(not(feature = "ssr"), allow(dead_code))]
fn validate_inquiry(input: InquiryInput) -> Result<InquiryInput, ServerFnError> {
    let email = parse_email(&input.email)?;
    let name = input.name.trim().to_string();
    let message = input.message.trim().to_string();
    if name.is_empty() {
        return Err(ServerFnError::new("Please enter your name."));
    }
    if message.len() < 10 {
        return Err(ServerFnError::new("Your message is too short."));
    }
    Ok(InquiryInput {
        product_slug: input.product_slug.filter(|s| !s.is_empty()),
        name,
        email,
        message,
    })
}

#[cfg(feature = "ssr")]
async fn insert(input: &InquiryInput) -> Result<(), ServerFnError> {
    let pool = expect_context::<sqlx::SqlitePool>();
    sqlx::query(
        "INSERT INTO inquiries (product_slug, name, email, message) \
         VALUES (?1, ?2, ?3, ?4)",
    )
    .bind(&input.product_slug)
    .bind(&input.name)
    .bind(&input.email)
    .bind(&input.message)
    .execute(&pool)
    .await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input(name: &str, email: &str, message: &str) -> InquiryInput {
        InquiryInput {
            product_slug: None,
            name: name.into(),
            email: email.into(),
            message: message.into(),
        }
    }

    #[test]
    fn rejects_empty_name() {
        assert!(validate_inquiry(input("", "me@x.co", "ten char message")).is_err());
    }

    #[test]
    fn rejects_short_message() {
        assert!(validate_inquiry(input("Yan", "me@x.co", "short")).is_err());
    }

    #[test]
    fn rejects_invalid_email() {
        assert!(validate_inquiry(input("Yan", "nope", "ten char message")).is_err());
    }

    #[test]
    fn accepts_valid_and_normalizes() {
        let r =
            validate_inquiry(input("  Yan  ", "Me@X.CO", "ten char message")).unwrap();
        assert_eq!(r.name, "Yan");
        assert_eq!(r.email, "me@x.co");
        assert_eq!(r.message, "ten char message");
    }

    #[test]
    fn empty_slug_becomes_none() {
        let mut i = input("Yan", "me@x.co", "ten char message");
        i.product_slug = Some(String::new());
        let r = validate_inquiry(i).unwrap();
        assert_eq!(r.product_slug, None);
    }
}
