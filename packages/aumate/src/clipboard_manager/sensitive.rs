//! Sensitive data detection using regex patterns

use regex::Regex;

use super::entry::SensitiveDataType;

/// Detector for sensitive data patterns
pub struct SensitiveDetector {
    password_regex: Regex,
    aws_key_regex: Regex,
    github_token_regex: Regex,
    stripe_key_regex: Regex,
    generic_api_key_regex: Regex,
    private_key_regex: Regex,
    visa_regex: Regex,
    mastercard_regex: Regex,
    amex_regex: Regex,
}

impl SensitiveDetector {
    /// Create a new sensitive data detector
    pub fn new() -> Self {
        Self {
            // Password patterns: "password:", "passwd=", "pwd:" etc.
            password_regex: Regex::new(
                r"(?i)(password|passwd|pwd|secret|token)\s*[:=]\s*\S+"
            ).unwrap(),

            // AWS Access Key ID: AKIA followed by 16 uppercase alphanumeric
            aws_key_regex: Regex::new(r"AKIA[0-9A-Z]{16}").unwrap(),

            // GitHub Personal Access Token: ghp_ followed by 36 alphanumeric
            github_token_regex: Regex::new(r"ghp_[a-zA-Z0-9]{36}").unwrap(),

            // Stripe API keys: sk_live_ or sk_test_ followed by alphanumeric
            stripe_key_regex: Regex::new(r"sk_(live|test)_[a-zA-Z0-9]{24,}").unwrap(),

            // Generic API key patterns
            generic_api_key_regex: Regex::new(
                r#"(?i)(api[_-]?key|apikey|api[_-]?secret)\s*[:=]\s*['"]?[a-zA-Z0-9]{16,}['"]?"#
            ).unwrap(),

            // Private key headers
            private_key_regex: Regex::new(
                r"-----BEGIN\s+(RSA|DSA|EC|OPENSSH|PGP)?\s*PRIVATE KEY-----"
            ).unwrap(),

            // Visa: starts with 4, 13-16 digits
            visa_regex: Regex::new(r"\b4[0-9]{12}(?:[0-9]{3})?\b").unwrap(),

            // Mastercard: starts with 51-55 or 2221-2720, 16 digits
            mastercard_regex: Regex::new(
                r"\b(?:5[1-5][0-9]{2}|222[1-9]|22[3-9][0-9]|2[3-6][0-9]{2}|27[01][0-9]|2720)[0-9]{12}\b"
            ).unwrap(),

            // American Express: starts with 34 or 37, 15 digits
            amex_regex: Regex::new(r"\b3[47][0-9]{13}\b").unwrap(),
        }
    }

    /// Detect sensitive data in text
    ///
    /// Returns the type of sensitive data found, or None if not sensitive.
    pub fn detect(&self, text: &str) -> Option<SensitiveDataType> {
        // Check for private keys first (most critical)
        if self.private_key_regex.is_match(text) {
            return Some(SensitiveDataType::PrivateKey);
        }

        // Check for API keys
        if self.aws_key_regex.is_match(text)
            || self.github_token_regex.is_match(text)
            || self.stripe_key_regex.is_match(text)
            || self.generic_api_key_regex.is_match(text)
        {
            return Some(SensitiveDataType::ApiKey);
        }

        // Check for passwords
        if self.password_regex.is_match(text) {
            return Some(SensitiveDataType::Password);
        }

        // Check for credit cards (with Luhn validation)
        if let Some(card_type) = self.detect_credit_card(text) {
            return Some(card_type);
        }

        None
    }

    /// Detect credit card numbers with Luhn validation
    fn detect_credit_card(&self, text: &str) -> Option<SensitiveDataType> {
        // Check each card type
        for regex in [&self.visa_regex, &self.mastercard_regex, &self.amex_regex] {
            for capture in regex.find_iter(text) {
                let card_number = capture.as_str();
                // Remove any spaces or dashes
                let digits: String = card_number.chars().filter(|c| c.is_ascii_digit()).collect();
                if self.luhn_check(&digits) {
                    return Some(SensitiveDataType::CreditCard);
                }
            }
        }
        None
    }

    /// Luhn algorithm for credit card validation
    fn luhn_check(&self, digits: &str) -> bool {
        if digits.len() < 13 || digits.len() > 19 {
            return false;
        }

        let mut sum = 0;
        let mut alternate = false;

        for c in digits.chars().rev() {
            if let Some(digit) = c.to_digit(10) {
                let mut n = digit;
                if alternate {
                    n *= 2;
                    if n > 9 {
                        n -= 9;
                    }
                }
                sum += n;
                alternate = !alternate;
            } else {
                return false;
            }
        }

        sum % 10 == 0
    }

    /// Check if text is sensitive (quick boolean check)
    pub fn is_sensitive(&self, text: &str) -> bool {
        self.detect(text).is_some()
    }
}

impl Default for SensitiveDetector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_detection() {
        let detector = SensitiveDetector::new();

        assert_eq!(detector.detect("password: mysecret123"), Some(SensitiveDataType::Password));
        assert_eq!(detector.detect("PASSWORD=secret"), Some(SensitiveDataType::Password));
        assert_eq!(detector.detect("db_pwd: abc123"), Some(SensitiveDataType::Password));
        assert!(detector.detect("just some normal text").is_none());
    }

    #[test]
    fn test_api_key_detection() {
        let detector = SensitiveDetector::new();

        // AWS key
        assert_eq!(detector.detect("AKIAIOSFODNN7EXAMPLE"), Some(SensitiveDataType::ApiKey));

        // GitHub token
        assert_eq!(
            detector.detect("ghp_1234567890abcdefghijklmnopqrstuvwxyz"),
            Some(SensitiveDataType::ApiKey)
        );

        // Stripe key pattern test - construct dynamically to avoid GitHub secret scanning
        let stripe_key = format!("sk_{}_1234567890abcdefghijklmn", "live");
        assert_eq!(detector.detect(&stripe_key), Some(SensitiveDataType::ApiKey));

        // Generic API key
        assert_eq!(
            detector.detect("api_key: abcdef1234567890abcd"),
            Some(SensitiveDataType::ApiKey)
        );
    }

    #[test]
    fn test_private_key_detection() {
        let detector = SensitiveDetector::new();

        assert_eq!(
            detector.detect("-----BEGIN RSA PRIVATE KEY-----\nMIIE..."),
            Some(SensitiveDataType::PrivateKey)
        );
        assert_eq!(
            detector.detect("-----BEGIN PRIVATE KEY-----"),
            Some(SensitiveDataType::PrivateKey)
        );
        assert_eq!(
            detector.detect("-----BEGIN OPENSSH PRIVATE KEY-----"),
            Some(SensitiveDataType::PrivateKey)
        );
    }

    #[test]
    fn test_credit_card_detection() {
        let detector = SensitiveDetector::new();

        // Valid Visa test number
        assert_eq!(detector.detect("Card: 4111111111111111"), Some(SensitiveDataType::CreditCard));

        // Valid Mastercard test number
        assert_eq!(detector.detect("MC: 5500000000000004"), Some(SensitiveDataType::CreditCard));

        // Invalid number (fails Luhn)
        assert!(detector.detect("4111111111111112").is_none());
    }

    #[test]
    fn test_luhn_algorithm() {
        let detector = SensitiveDetector::new();

        assert!(detector.luhn_check("4111111111111111")); // Valid Visa
        assert!(detector.luhn_check("5500000000000004")); // Valid MC
        assert!(detector.luhn_check("378282246310005")); // Valid Amex
        assert!(!detector.luhn_check("4111111111111112")); // Invalid
        assert!(!detector.luhn_check("123")); // Too short
    }

    #[test]
    fn test_non_sensitive() {
        let detector = SensitiveDetector::new();

        assert!(detector.detect("Hello, World!").is_none());
        assert!(detector.detect("Just a normal email: test@example.com").is_none());
        assert!(detector.detect("Some code: let x = 42;").is_none());
    }
}
