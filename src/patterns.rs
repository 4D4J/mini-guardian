use regex::Regex;

#[derive(Debug, Clone)]
pub struct SecretPattern {
    pub name: String,
    pub pattern: Regex,
    pub description: String,
}

impl SecretPattern {
    pub fn new(name: &str, pattern: &str, description: &str) -> Self {
        Self {
            name: name.to_string(),
            pattern: Regex::new(pattern).expect("Invalid regex pattern"),
            description: description.to_string(),
        }
    }
}

pub fn get_default_patterns() -> Vec<SecretPattern> {
    vec![
        SecretPattern::new(
            "AWS Access Key ID",
            r"AKIA[0-9A-Z]{16}",
            "Amazon Web Services access key",
        ),
        SecretPattern::new(
            "AWS Secret Key",
            r#"(?i)aws(.{0,20})?['"][0-9a-zA-Z/+]{40}['"]"#,
            "Amazon Web Services secret key",
        ),
        SecretPattern::new(
            "GitHub Token",
            r"gh[pousr]_[A-Za-z0-9_]{36,255}",
            "GitHub Personal Access Token",
        ),
        SecretPattern::new(
            "GitHub OAuth",
            r"gho_[A-Za-z0-9_]{36,255}",
            "GitHub OAuth Access Token",
        ),
        SecretPattern::new(
            "Private Key",
            r"-----BEGIN\s+(RSA|EC|DSA|OPENSSH|PGP)?\s*PRIVATE KEY-----",
            "Private key file",
        ),
        SecretPattern::new(
            "Generic API Key",
            r#"(?i)(api[_-]?key|apikey)\s*[:=]\s*['"]?[a-zA-Z0-9_\-]{20,}['"]?"#,
            "Generic API key pattern",
        ),
        SecretPattern::new(
            "JWT Token",
            r"eyJ[A-Za-z0-9-_]+\.eyJ[A-Za-z0-9-_]+\.[A-Za-z0-9-_]+",
            "JSON Web Token",
        ),
        SecretPattern::new(
            "Slack Token",
            r"xox[baprs]-[0-9]{10,13}-[0-9]{10,13}[a-zA-Z0-9-]*",
            "Slack API Token",
        ),
        SecretPattern::new(
            "Slack Webhook",
            r"https://hooks\.slack\.com/services/T[A-Z0-9]+/B[A-Z0-9]+/[a-zA-Z0-9]+",
            "Slack Webhook URL",
        ),
        SecretPattern::new(
            "Google API Key",
            r"AIza[0-9A-Za-z\-_]{35}",
            "Google API Key",
        ),
        SecretPattern::new(
            "Stripe Secret Key",
            r"sk_live_[0-9a-zA-Z]{24,}",
            "Stripe Secret API Key",
        ),
        SecretPattern::new(
            "Stripe Publishable Key",
            r"pk_live_[0-9a-zA-Z]{24,}",
            "Stripe Publishable API Key",
        ),
        SecretPattern::new(
            "Discord Token",
            r"[MN][A-Za-z\d]{23,}\.[\w-]{6}\.[\w-]{27}",
            "Discord Bot Token",
        ),
        SecretPattern::new(
            "Password in URL",
            r"[a-zA-Z]{3,10}://[^/\s:@]{1,100}:[^/\s:@]{1,100}@[^\s/]+",
            "Password embedded in URL",
        ),
        SecretPattern::new(
            "Generic Password",
            r#"(?i)(password|passwd|pwd)\s*[:=]\s*['"][^'"]{8,}['"]"#,
            "Hardcoded password",
        ),
        SecretPattern::new(
            "Heroku API Key",
            r"[h|H][e|E][r|R][o|O][k|K][u|U].{0,30}[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}",
            "Heroku API Key",
        ),
        SecretPattern::new(
            "SendGrid API Key",
            r"SG\.[a-zA-Z0-9_-]{22}\.[a-zA-Z0-9_-]{43}",
            "SendGrid API Key",
        ),
        SecretPattern::new(
            "Twilio API Key",
            r"SK[a-f0-9]{32}",
            "Twilio API Key",
        ),
        SecretPattern::new(
            "npm Token",
            r"npm_[A-Za-z0-9]{36}",
            "npm Access Token",
        ),
        SecretPattern::new(
            "Vite Token",
            r"vite_[a-zA-Z0-9]{32,}",
            "Vite API Token",
        ),
        SecretPattern::new(
            "Supabase Anon Key",
            r"eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9\.[a-zA-Z0-9_-]+\.[a-zA-Z0-9_-]+",
            "Supabase Anonymous/Public Key (JWT)",
        ),
        SecretPattern::new(
            "Supabase Service Key",
            r"sbp_[a-f0-9]{40}",
            "Supabase Service Role Key",
        ),
    ]
}
