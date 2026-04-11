use nanoid::nanoid;

/// Character set for short codes: a-z, A-Z, 0-9 (62 chars)
const ALPHABET: [char; 62] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
    'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9',
];

/// Generates a random short code using nanoid with base62 alphabet.
pub fn generate_short_code(length: usize) -> String {
    nanoid!(length, &ALPHABET)
}

/// Validates a custom alias provided by the user.
/// Rules: 3-16 chars, only alphanumeric and hyphens, cannot start/end with hyphen.
pub fn validate_custom_code(code: &str, max_length: usize) -> Result<(), String> {
    if code.len() < 3 {
        return Err("Custom code must be at least 3 characters".to_string());
    }

    if code.len() > max_length {
        return Err(format!(
            "Custom code must be at most {} characters",
            max_length
        ));
    }

    if code.starts_with('-') || code.ends_with('-') {
        return Err("Custom code cannot start or end with a hyphen".to_string());
    }

    if !code.chars().all(|c| c.is_ascii_alphanumeric() || c == '-') {
        return Err("Custom code can only contain alphanumeric characters and hyphens".to_string());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_code_with_correct_length() {
        let code = generate_short_code(7);
        assert_eq!(code.len(), 7);
    }

    #[test]
    fn generates_code_with_valid_chars() {
        let code = generate_short_code(100);
        assert!(code.chars().all(|c| ALPHABET.contains(&c)));
    }

    #[test]
    fn generates_unique_codes() {
        let codes: Vec<String> = (0..1000).map(|_| generate_short_code(7)).collect();
        let unique: std::collections::HashSet<&String> = codes.iter().collect();
        // With 62^7 combinations, 1000 codes should all be unique
        assert_eq!(codes.len(), unique.len());
    }

    #[test]
    fn validates_good_custom_codes() {
        assert!(validate_custom_code("my-link", 16).is_ok());
        assert!(validate_custom_code("abc", 16).is_ok());
        assert!(validate_custom_code("MyCode123", 16).is_ok());
    }

    #[test]
    fn rejects_too_short_code() {
        assert!(validate_custom_code("ab", 16).is_err());
    }

    #[test]
    fn rejects_too_long_code() {
        assert!(validate_custom_code("a".repeat(17).as_str(), 16).is_err());
    }

    #[test]
    fn rejects_hyphen_at_edges() {
        assert!(validate_custom_code("-abc", 16).is_err());
        assert!(validate_custom_code("abc-", 16).is_err());
    }

    #[test]
    fn rejects_invalid_characters() {
        assert!(validate_custom_code("my link", 16).is_err());
        assert!(validate_custom_code("my@link", 16).is_err());
        assert!(validate_custom_code("türkçe", 16).is_err());
    }
}
