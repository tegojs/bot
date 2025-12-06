//! Window positioning actions
//!
//! Defines all the window management actions like "Left Half", "Right Half", etc.

/// Screen bounds for calculating window positions
#[derive(Debug, Clone, Copy)]
pub struct ScreenBounds {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

/// Target bounds for a window after applying an action
#[derive(Debug, Clone, Copy)]
pub struct TargetBounds {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

/// Current window bounds (for actions that need to know current state)
#[derive(Debug, Clone, Copy)]
pub struct CurrentBounds {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

/// A window management action
#[derive(Debug, Clone)]
pub struct WindowAction {
    /// Unique identifier
    pub id: &'static str,
    /// Display name shown in palette
    pub name: &'static str,
    /// Alternative names for search matching
    pub aliases: &'static [&'static str],
    /// Category for grouping
    pub category: &'static str,
}

impl WindowAction {
    /// Calculate target bounds for this action
    pub fn calculate(&self, screen: ScreenBounds, current: Option<CurrentBounds>) -> TargetBounds {
        match self.id {
            "left_half" => TargetBounds {
                x: screen.x,
                y: screen.y,
                width: screen.width / 2,
                height: screen.height,
            },
            "right_half" => TargetBounds {
                x: screen.x + (screen.width / 2) as i32,
                y: screen.y,
                width: screen.width / 2,
                height: screen.height,
            },
            "top_half" => TargetBounds {
                x: screen.x,
                y: screen.y,
                width: screen.width,
                height: screen.height / 2,
            },
            "bottom_half" => TargetBounds {
                x: screen.x,
                y: screen.y + (screen.height / 2) as i32,
                width: screen.width,
                height: screen.height / 2,
            },
            "top_left" => TargetBounds {
                x: screen.x,
                y: screen.y,
                width: screen.width / 2,
                height: screen.height / 2,
            },
            "top_right" => TargetBounds {
                x: screen.x + (screen.width / 2) as i32,
                y: screen.y,
                width: screen.width / 2,
                height: screen.height / 2,
            },
            "bottom_left" => TargetBounds {
                x: screen.x,
                y: screen.y + (screen.height / 2) as i32,
                width: screen.width / 2,
                height: screen.height / 2,
            },
            "bottom_right" => TargetBounds {
                x: screen.x + (screen.width / 2) as i32,
                y: screen.y + (screen.height / 2) as i32,
                width: screen.width / 2,
                height: screen.height / 2,
            },
            "left_third" => TargetBounds {
                x: screen.x,
                y: screen.y,
                width: screen.width / 3,
                height: screen.height,
            },
            "center_third" => TargetBounds {
                x: screen.x + (screen.width / 3) as i32,
                y: screen.y,
                width: screen.width / 3,
                height: screen.height,
            },
            "right_third" => TargetBounds {
                x: screen.x + (screen.width * 2 / 3) as i32,
                y: screen.y,
                width: screen.width / 3,
                height: screen.height,
            },
            "left_two_thirds" => TargetBounds {
                x: screen.x,
                y: screen.y,
                width: screen.width * 2 / 3,
                height: screen.height,
            },
            "right_two_thirds" => TargetBounds {
                x: screen.x + (screen.width / 3) as i32,
                y: screen.y,
                width: screen.width * 2 / 3,
                height: screen.height,
            },
            "maximize" => TargetBounds {
                x: screen.x,
                y: screen.y,
                width: screen.width,
                height: screen.height,
            },
            "almost_maximize" => {
                let margin_x = (screen.width as f32 * 0.05) as u32;
                let margin_y = (screen.height as f32 * 0.05) as u32;
                TargetBounds {
                    x: screen.x + margin_x as i32,
                    y: screen.y + margin_y as i32,
                    width: screen.width - margin_x * 2,
                    height: screen.height - margin_y * 2,
                }
            }
            "center" => {
                // Keep current size, center on screen
                if let Some(curr) = current {
                    let center_x = screen.x + (screen.width as i32 - curr.width as i32) / 2;
                    let center_y = screen.y + (screen.height as i32 - curr.height as i32) / 2;
                    TargetBounds {
                        x: center_x,
                        y: center_y,
                        width: curr.width,
                        height: curr.height,
                    }
                } else {
                    // No current bounds, use a reasonable default (50% of screen)
                    let width = screen.width / 2;
                    let height = screen.height / 2;
                    TargetBounds {
                        x: screen.x + (screen.width - width) as i32 / 2,
                        y: screen.y + (screen.height - height) as i32 / 2,
                        width,
                        height,
                    }
                }
            }
            _ => TargetBounds {
                x: screen.x,
                y: screen.y,
                width: screen.width,
                height: screen.height,
            },
        }
    }

    /// Check if this action matches the search query
    pub fn matches(&self, query: &str) -> bool {
        if query.is_empty() {
            return true;
        }
        let query_lower = query.to_lowercase();
        if self.name.to_lowercase().contains(&query_lower) {
            return true;
        }
        self.aliases.iter().any(|alias| alias.to_lowercase().contains(&query_lower))
    }

    /// Get match score (lower is better)
    pub fn match_score(&self, query: &str) -> usize {
        if query.is_empty() {
            return 100; // Default order
        }
        let query_lower = query.to_lowercase();
        let name_lower = self.name.to_lowercase();

        // Exact match
        if name_lower == query_lower {
            return 0;
        }
        // Starts with
        if name_lower.starts_with(&query_lower) {
            return 1;
        }
        // Alias exact match
        for alias in self.aliases {
            if alias.to_lowercase() == query_lower {
                return 2;
            }
        }
        // Alias starts with
        for alias in self.aliases {
            if alias.to_lowercase().starts_with(&query_lower) {
                return 3;
            }
        }
        // Contains
        if name_lower.contains(&query_lower) {
            return 4;
        }
        // Alias contains
        for alias in self.aliases {
            if alias.to_lowercase().contains(&query_lower) {
                return 5;
            }
        }
        100 // No match
    }
}

/// All available window actions
pub static WINDOW_ACTIONS: &[WindowAction] = &[
    WindowAction {
        id: "left_half",
        name: "Left Half",
        aliases: &["left", "l", "lh"],
        category: "Halves",
    },
    WindowAction {
        id: "right_half",
        name: "Right Half",
        aliases: &["right", "r", "rh"],
        category: "Halves",
    },
    WindowAction {
        id: "top_half",
        name: "Top Half",
        aliases: &["top", "t", "th"],
        category: "Halves",
    },
    WindowAction {
        id: "bottom_half",
        name: "Bottom Half",
        aliases: &["bottom", "b", "bh"],
        category: "Halves",
    },
    WindowAction {
        id: "top_left",
        name: "Top Left Quarter",
        aliases: &["tl", "topleft", "top-left"],
        category: "Quarters",
    },
    WindowAction {
        id: "top_right",
        name: "Top Right Quarter",
        aliases: &["tr", "topright", "top-right"],
        category: "Quarters",
    },
    WindowAction {
        id: "bottom_left",
        name: "Bottom Left Quarter",
        aliases: &["bl", "bottomleft", "bottom-left"],
        category: "Quarters",
    },
    WindowAction {
        id: "bottom_right",
        name: "Bottom Right Quarter",
        aliases: &["br", "bottomright", "bottom-right"],
        category: "Quarters",
    },
    WindowAction {
        id: "left_third",
        name: "Left Third",
        aliases: &["l3", "first third"],
        category: "Thirds",
    },
    WindowAction {
        id: "center_third",
        name: "Center Third",
        aliases: &["c3", "middle third"],
        category: "Thirds",
    },
    WindowAction {
        id: "right_third",
        name: "Right Third",
        aliases: &["r3", "last third"],
        category: "Thirds",
    },
    WindowAction {
        id: "left_two_thirds",
        name: "Left Two Thirds",
        aliases: &["l23", "first two thirds"],
        category: "Thirds",
    },
    WindowAction {
        id: "right_two_thirds",
        name: "Right Two Thirds",
        aliases: &["r23", "last two thirds"],
        category: "Thirds",
    },
    WindowAction {
        id: "maximize",
        name: "Maximize",
        aliases: &["max", "full", "m", "fullscreen"],
        category: "Size",
    },
    WindowAction {
        id: "almost_maximize",
        name: "Almost Maximize",
        aliases: &["almost", "am", "near max"],
        category: "Size",
    },
    WindowAction {
        id: "center",
        name: "Center",
        aliases: &["c", "middle", "center window"],
        category: "Position",
    },
];

/// Filter actions by search query
pub fn filter_actions(query: &str) -> Vec<&'static WindowAction> {
    let mut matches: Vec<_> = WINDOW_ACTIONS
        .iter()
        .filter(|action| action.matches(query))
        .map(|action| (action.match_score(query), action))
        .collect();

    matches.sort_by_key(|(score, _)| *score);
    matches.into_iter().map(|(_, action)| action).collect()
}

/// Get an action by ID
pub fn get_action_by_id(id: &str) -> Option<&'static WindowAction> {
    WINDOW_ACTIONS.iter().find(|action| action.id == id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_actions_empty() {
        let results = filter_actions("");
        assert_eq!(results.len(), WINDOW_ACTIONS.len());
    }

    #[test]
    fn test_filter_actions_left() {
        let results = filter_actions("left");
        assert!(results.iter().any(|a| a.id == "left_half"));
        assert!(results.iter().any(|a| a.id == "left_third"));
        assert!(results.iter().any(|a| a.id == "left_two_thirds"));
    }

    #[test]
    fn test_filter_actions_alias() {
        let results = filter_actions("l3");
        assert!(!results.is_empty());
        assert_eq!(results[0].id, "left_third");
    }

    #[test]
    fn test_action_calculate_left_half() {
        let screen = ScreenBounds { x: 0, y: 0, width: 1920, height: 1080 };
        let action = get_action_by_id("left_half").unwrap();
        let target = action.calculate(screen, None);
        assert_eq!(target.x, 0);
        assert_eq!(target.y, 0);
        assert_eq!(target.width, 960);
        assert_eq!(target.height, 1080);
    }

    #[test]
    fn test_action_calculate_center() {
        let screen = ScreenBounds { x: 0, y: 0, width: 1920, height: 1080 };
        let current = CurrentBounds { x: 100, y: 100, width: 800, height: 600 };
        let action = get_action_by_id("center").unwrap();
        let target = action.calculate(screen, Some(current));
        assert_eq!(target.x, (1920 - 800) / 2);
        assert_eq!(target.y, (1080 - 600) / 2);
        assert_eq!(target.width, 800);
        assert_eq!(target.height, 600);
    }
}
