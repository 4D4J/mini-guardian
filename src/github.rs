use octocrab::Octocrab;
use serde::{Deserialize, Serialize};
use base64::Engine;
use base64::engine::general_purpose::STANDARD;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepoInfo {
    pub name: String,
    pub full_name: String,
    pub private: bool,
    pub default_branch: String,
}

#[derive(Debug, Clone)]
pub struct BranchInfo {
    pub name: String,
}

pub struct GitHubClient {
    client: Octocrab,
}

impl GitHubClient {
    pub fn new(token: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let client = Octocrab::builder()
            .personal_token(token.to_string())
            .build()?;
        Ok(Self { client })
    }

    pub async fn list_repos(&self) -> Result<Vec<RepoInfo>, Box<dyn std::error::Error>> {
        let mut repos = Vec::new();
        let mut page = 1u8;

        loop {
            let result = self
                .client
                .current()
                .list_repos_for_authenticated_user()
                .per_page(100)
                .page(page)
                .send()
                .await?;

            if result.items.is_empty() {
                break;
            }

            for repo in result.items {
                repos.push(RepoInfo {
                    name: repo.name.clone(),
                    full_name: repo.full_name.unwrap_or(repo.name),
                    private: repo.private.unwrap_or(false),
                    default_branch: repo.default_branch.unwrap_or_else(|| "main".to_string()),
                });
            }

            page += 1;
            if page > 100 {
                break;
            }
        }

        Ok(repos)
    }

    pub async fn list_branches(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<Vec<BranchInfo>, Box<dyn std::error::Error>> {
        let mut branches = Vec::new();
        let mut page = 1u8;

        loop {
            let result = self
                .client
                .repos(owner, repo)
                .list_branches()
                .per_page(100)
                .page(page)
                .send()
                .await?;

            if result.items.is_empty() {
                break;
            }

            for branch in result.items {
                branches.push(BranchInfo {
                    name: branch.name,
                });
            }

            page += 1;
            if page > 10 {
                break;
            }
        }

        Ok(branches)
    }

    pub async fn get_repo_files(
        &self,
        owner: &str,
        repo: &str,
        path: &str,
        branch: Option<&str>,
    ) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut files = Vec::new();
        self.get_files_recursive(owner, repo, path, branch, &mut files).await?;
        Ok(files)
    }

    async fn get_files_recursive(
        &self,
        owner: &str,
        repo: &str,
        path: &str,
        branch: Option<&str>,
        files: &mut Vec<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let repo_handler = self.client.repos(owner, repo);
        let content = match branch {
            Some(b) => repo_handler.get_content().path(path).r#ref(b).send().await?,
            None => repo_handler.get_content().path(path).send().await?,
        };

        if content.items.len() == 1 && content.items[0].r#type == "file" {
            files.push(path.to_string());
        } else {
            for item in &content.items {
                let item_path = item.path.clone();
                match item.r#type.as_str() {
                    "file" => {
                        if Self::is_scannable_file(&item_path) {
                            files.push(item_path);
                        }
                    }
                    "dir" => {
                        if !Self::should_skip_dir(&item_path) {
                            Box::pin(self.get_files_recursive(owner, repo, &item_path, branch, files)).await?;
                        }
                    }
                    _ => {}
                }
            }
        }

        Ok(())
    }

    fn is_scannable_file(path: &str) -> bool {
        let scannable_extensions = [
            ".rs", ".py", ".js", ".ts", ".jsx", ".tsx", ".go", ".java", ".rb", ".php",
            ".cs", ".cpp", ".c", ".h", ".hpp", ".swift", ".kt", ".scala", ".sh", ".bash",
            ".env", ".yml", ".yaml", ".json", ".toml", ".xml", ".ini", ".cfg", ".conf",
            ".properties", ".md", ".txt", ".sql", ".dockerfile", ".tf", ".tfvars",
        ];
        
        let path_lower = path.to_lowercase();
        
        let secret_filenames = [
            ".env", ".env.local", ".env.development", ".env.production",
            "credentials", "secrets", "config", ".npmrc", ".pypirc",
        ];
        
        for filename in secret_filenames {
            if path_lower.ends_with(filename) {
                return true;
            }
        }
        
        scannable_extensions.iter().any(|ext| path_lower.ends_with(ext))
    }

    fn should_skip_dir(path: &str) -> bool {
        let skip_dirs = [
            "node_modules", ".git", "vendor", "target", "dist", "build",
            "__pycache__", ".venv", "venv", ".idea", ".vscode", "coverage",
            ".next", ".nuxt", "out", "bin", "obj", "packages",
        ];
        
        skip_dirs.iter().any(|dir| path.contains(dir))
    }

    pub async fn get_file_content(
        &self,
        owner: &str,
        repo: &str,
        path: &str,
        branch: Option<&str>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let repo_handler = self.client.repos(owner, repo);
        let content = match branch {
            Some(b) => repo_handler.get_content().path(path).r#ref(b).send().await?,
            None => repo_handler.get_content().path(path).send().await?,
        };

        if let Some(item) = content.items.first() {
            if let Some(ref encoded_content) = item.content {
                let cleaned: String = encoded_content.chars().filter(|c| !c.is_whitespace()).collect();
                let decoded = STANDARD.decode(&cleaned)?;
                return Ok(String::from_utf8_lossy(&decoded).to_string());
            }
        }
        
        Ok(String::new())
    }
}
