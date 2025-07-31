use regex::Regex;

/// 验证邮箱格式
pub fn is_valid_email(email: &str) -> bool {
    let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
    email_regex.is_match(email)
}

/// 验证用户名格式（只允许字母、数字和下划线，3-20个字符）
pub fn is_valid_username(username: &str) -> bool {
    let username_regex = Regex::new(r"^[a-zA-Z0-9_]{3,20}$").unwrap();
    username_regex.is_match(username)
}

/// 验证密码强度（至少8个字符，包含字母和数字）
pub fn is_strong_password(password: &str) -> bool {
    if password.len() < 8 {
        return false;
    }

    let has_letter = password.chars().any(|c| c.is_alphabetic());
    let has_digit = password.chars().any(|c| c.is_numeric());

    has_letter && has_digit
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_email_validation() {
        assert!(is_valid_email("test@example.com"));
        assert!(is_valid_email("user.name+tag@domain.co.uk"));
        assert!(!is_valid_email("invalid.email"));
        assert!(!is_valid_email("@domain.com"));
        assert!(!is_valid_email("user@"));
    }

    #[test]
    fn test_username_validation() {
        assert!(is_valid_username("user123"));
        assert!(is_valid_username("test_user"));
        assert!(!is_valid_username("ab")); // too short
        assert!(!is_valid_username("user-name")); // contains dash
        assert!(!is_valid_username("user@name")); // contains special char
    }

    #[test]
    fn test_password_strength() {
        assert!(is_strong_password("password123"));
        assert!(is_strong_password("MyPass123"));
        assert!(!is_strong_password("password")); // no digits
        assert!(!is_strong_password("12345678")); // no letters
        assert!(!is_strong_password("Pass1")); // too short
    }
}
