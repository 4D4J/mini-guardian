use crate::patterns::{get_default_patterns, SecretPattern};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Finding {
    pub file_path: String,
    pub line_number: usize,
    pub line_content: String,
    pub secret_type: String,
    pub matched_text: String,
}

impl Finding {
    pub fn masked_text(&self) -> String {
        let len = self.matched_text.len();
        if len <= 8 {
            "*".repeat(len)
        } else {
            format!(
                "{}...{}",
                &self.matched_text[..4],
                &self.matched_text[len - 4..]
            )
        }
    }
}

pub struct Scanner {
    patterns: Vec<SecretPattern>,
}

impl Scanner {
    pub fn new() -> Self {
        Self {
            patterns: get_default_patterns(),
        }
    }

    pub fn patterns(&self) -> &[SecretPattern] {
        &self.patterns
    }

    pub fn scan_content(&self, file_path: &str, content: &str) -> Vec<Finding> {
        let mut findings = Vec::new();

        for (line_number, line) in content.lines().enumerate() {
            for pattern in &self.patterns {
                if let Some(matched) = pattern.pattern.find(line) {
                    findings.push(Finding {
                        file_path: file_path.to_string(),
                        line_number: line_number + 1,
                        line_content: line.to_string(),
                        secret_type: pattern.name.clone(),
                        matched_text: matched.as_str().to_string(),
                    });
                }
            }
        }

        findings
    }
}

impl Default for Scanner {
    fn default() -> Self {
        Self::new()
    }
}
