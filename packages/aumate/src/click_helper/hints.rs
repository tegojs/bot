//! Two-tier hint character generation for Click Helper
//!
//! Generates hint labels for UI elements using a two-tier system:
//! - If <=26 elements: single letters (a-z)
//! - If >26 elements: symbols + letters (e.g., ",a", ".b")

use super::config::ClickHelperConfig;

/// A hint label for a clickable element
#[derive(Debug, Clone)]
pub struct HintLabel {
    /// The hint string to display (e.g., ",a" or just "a")
    pub label: String,
    /// Associated element index
    pub element_index: usize,
    /// Whether this hint uses two tiers
    pub is_two_tier: bool,
}

/// Generator for two-tier hint labels
pub struct HintGenerator {
    tier1_chars: Vec<char>,
    tier2_chars: Vec<char>,
}

impl HintGenerator {
    /// Create a new hint generator from config
    pub fn new(config: &ClickHelperConfig) -> Self {
        Self { tier1_chars: config.tier1_chars_vec(), tier2_chars: config.tier2_chars_vec() }
    }

    /// Create a hint generator with custom characters
    pub fn with_chars(tier1: &str, tier2: &str) -> Self {
        Self { tier1_chars: tier1.chars().collect(), tier2_chars: tier2.chars().collect() }
    }

    /// Get the maximum number of elements this generator can handle
    pub fn max_elements(&self) -> usize {
        self.tier1_chars.len() * self.tier2_chars.len()
    }

    /// Generate hint labels for a given number of elements
    pub fn generate(&self, count: usize) -> Vec<HintLabel> {
        let tier2_count = self.tier2_chars.len();

        if count == 0 {
            return Vec::new();
        }

        if count <= tier2_count {
            // Single-tier: just use tier2 chars (a-z)
            self.tier2_chars
                .iter()
                .take(count)
                .enumerate()
                .map(|(i, c)| HintLabel {
                    label: c.to_string(),
                    element_index: i,
                    is_two_tier: false,
                })
                .collect()
        } else {
            // Two-tier: use tier1 + tier2 chars
            let mut labels = Vec::with_capacity(count);
            let tier1_needed = count.div_ceil(tier2_count);

            for (group_idx, tier1_char) in self.tier1_chars.iter().take(tier1_needed).enumerate() {
                let start = group_idx * tier2_count;
                let end = (start + tier2_count).min(count);

                for (i, tier2_char) in self.tier2_chars.iter().take(end - start).enumerate() {
                    labels.push(HintLabel {
                        label: format!("{}{}", tier1_char, tier2_char),
                        element_index: start + i,
                        is_two_tier: true,
                    });
                }
            }
            labels
        }
    }

    /// Find hints that match the given prefix
    pub fn filter_by_prefix<'a>(hints: &'a [HintLabel], prefix: &str) -> Vec<&'a HintLabel> {
        hints.iter().filter(|h| h.label.starts_with(prefix)).collect()
    }

    /// Find the exact matching hint
    pub fn find_exact<'a>(hints: &'a [HintLabel], label: &str) -> Option<&'a HintLabel> {
        hints.iter().find(|h| h.label == label)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn default_generator() -> HintGenerator {
        HintGenerator::with_chars(",./;'[]", "abcdefghijklmnopqrstuvwxyz")
    }

    #[test]
    fn test_single_tier_small() {
        let generator = default_generator();
        let hints = generator.generate(5);

        assert_eq!(hints.len(), 5);
        assert_eq!(hints[0].label, "a");
        assert_eq!(hints[1].label, "b");
        assert_eq!(hints[4].label, "e");

        for hint in &hints {
            assert!(!hint.is_two_tier);
        }
    }

    #[test]
    fn test_single_tier_max() {
        let generator = default_generator();
        let hints = generator.generate(26);

        assert_eq!(hints.len(), 26);
        assert_eq!(hints[0].label, "a");
        assert_eq!(hints[25].label, "z");
    }

    #[test]
    fn test_two_tier() {
        let generator = default_generator();
        let hints = generator.generate(30);

        assert_eq!(hints.len(), 30);

        // First group: ,a through ,z
        assert_eq!(hints[0].label, ",a");
        assert_eq!(hints[25].label, ",z");

        // Second group: .a through .d
        assert_eq!(hints[26].label, ".a");
        assert_eq!(hints[29].label, ".d");

        for hint in &hints {
            assert!(hint.is_two_tier);
        }
    }

    #[test]
    fn test_filter_by_prefix() {
        let generator = default_generator();
        let hints = generator.generate(60);

        // Filter by ","
        let filtered = HintGenerator::filter_by_prefix(&hints, ",");
        assert_eq!(filtered.len(), 26);
        assert!(filtered.iter().all(|h| h.label.starts_with(',')));

        // Filter by ",a"
        let filtered = HintGenerator::filter_by_prefix(&hints, ",a");
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].label, ",a");
    }

    #[test]
    fn test_find_exact() {
        let generator = default_generator();
        let hints = generator.generate(30);

        let found = HintGenerator::find_exact(&hints, ",a");
        assert!(found.is_some());
        assert_eq!(found.unwrap().element_index, 0);

        let found = HintGenerator::find_exact(&hints, ".b");
        assert!(found.is_some());
        assert_eq!(found.unwrap().element_index, 27);

        let not_found = HintGenerator::find_exact(&hints, "xyz");
        assert!(not_found.is_none());
    }

    #[test]
    fn test_empty() {
        let generator = default_generator();
        let hints = generator.generate(0);
        assert!(hints.is_empty());
    }

    #[test]
    fn test_max_elements() {
        let generator = default_generator();
        // 7 tier1 chars * 26 tier2 chars = 182
        assert_eq!(generator.max_elements(), 182);
    }
}
