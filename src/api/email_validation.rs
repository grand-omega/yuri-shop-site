use leptos::prelude::*;

#[cfg_attr(not(feature = "ssr"), allow(dead_code))]
pub fn parse_email(input: &str) -> Result<String, ServerFnError> {
    let trimmed = input.trim().to_lowercase();
    if !trimmed.contains('@') || trimmed.len() < 5 {
        return Err(ServerFnError::new("Invalid email address."));
    }
    Ok(trimmed)
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
