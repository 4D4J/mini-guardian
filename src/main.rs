mod github;
mod patterns;
mod reporter;
mod scanner;

use clap::{Parser, Subcommand};
use colored::*;
use dotenv::dotenv;
use std::env;

use github::GitHubClient;
use reporter::{print_findings, print_findings_json, print_scan_summary};
use scanner::Scanner;

#[derive(Parser)]
#[command(name = "mini-guardian")]
#[command(author = "4D4J")]
#[command(version = "0.1.0")]
#[command(about = "Scan your GitHub repos for secrets", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Output results in JSON format
    #[arg(short, long, global = true)]
    json: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// List all your GitHub repositories
    Repos,

    /// Scan a specific repository for secrets
    Scan {
        /// Repository name (owner/repo or just repo name)
        repo: String,
    },

    /// Scan all your repositories for secrets
    ScanAll {
        #[arg(long, help = "Scan only private repositories")]
        private_only: bool,
    },

    /// Show available secret detection patterns
    Patterns,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let cli = Cli::parse();

    let token = match env::var("GITHUB_TOKEN") {
        Ok(t) => t,
        Err(_) => {
            eprintln!(
                "{} GITHUB_TOKEN not found. Set it in .env or environment.",
                "Error:".red().bold()
            );
            eprintln!("  export GITHUB_TOKEN=ghp_your_token_here");
            std::process::exit(1);
        }
    };

    let github = match GitHubClient::new(&token) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{} Failed to create GitHub client: {}", "Error:".red().bold(), e);
            std::process::exit(1);
        }
    };

    match cli.command {
        Commands::Repos => {
            println!("{}", " Fetching your repositories...".cyan());
            match github.list_repos().await {
                Ok(repos) => {
                    println!(
                        "\n{} Found {} repositories:\n",
                        "✓".green().bold(),
                        repos.len().to_string().cyan()
                    );
                    for repo in repos {
                        let visibility = if repo.private {
                            "private".yellow()
                        } else {
                            "public".green()
                        };
                        println!("  {} [{}]", repo.full_name.white(), visibility);
                    }
                }
                Err(e) => {
                    eprintln!("{} Failed to list repos: {}", "Error:".red().bold(), e);
                }
            }
        }

        Commands::Scan { repo } => {
            let (owner, repo_name) = parse_repo(&repo, &github).await;
            scan_repository(&github, &owner, &repo_name, cli.json).await;
        }

        Commands::ScanAll { private_only } => {
            let json = cli.json;
            if !json {
                println!("{}", " Scanning all repositories...".cyan());
            }
            match github.list_repos().await {
                Ok(repos) => {
                    let scanner = Scanner::new();
                    let mut total_files = 0;
                    let mut total_findings = 0;
                    let mut all_findings = Vec::new();

                    let repos_to_scan: Vec<_> = if private_only {
                        repos.into_iter().filter(|r| r.private).collect()
                    } else {
                        repos
                    };

                    for repo in &repos_to_scan {
                        let parts: Vec<&str> = repo.full_name.split('/').collect();
                        if parts.len() == 2 {
                            let (owner, name) = (parts[0], parts[1]);
                            
                            if !json {
                                println!("\nScanning {}...", repo.full_name.cyan());
                            }

                            let branches = match github.list_branches(owner, name).await {
                                Ok(b) => b,
                                Err(_) => {
                                    if !json {
                                        eprintln!("  {} Could not list branches", "!".yellow());
                                    }
                                    continue;
                                }
                            };

                            if !json {
                                let branch_names: Vec<&str> = branches.iter().map(|b| b.name.as_str()).collect();
                                println!("  Branches ({}): {}", branches.len(), branch_names.join(", ").yellow());
                            }

                            for branch in &branches {
                                match github.get_repo_files(owner, name, "", Some(&branch.name)).await {
                                    Ok(files) => {
                                        total_files += files.len();
                                        for file_path in files {
                                            if let Ok(content) = github.get_file_content(owner, name, &file_path, Some(&branch.name)).await {
                                                let findings = scanner.scan_content(&file_path, &content);
                                                total_findings += findings.len();
                                                
                                                for mut finding in findings {
                                                    finding.file_path = format!("{}/[{}] {}", repo.full_name, branch.name, finding.file_path);
                                                    all_findings.push(finding);
                                                }
                                            }
                                        }
                                    }
                                    Err(_) => continue,
                                }
                            }
                        }
                    }

                    if json {
                        print_findings_json(&all_findings);
                    } else {
                        if !all_findings.is_empty() {
                            print_findings(&all_findings, "all repositories");
                        }
                        print_scan_summary(repos_to_scan.len(), total_files, total_findings);
                    }
                }
                Err(e) => {
                    eprintln!("{} Failed to list repos: {}", "Error:".red().bold(), e);
                }
            }
        }

        Commands::Patterns => {
            let scanner = Scanner::new();
            println!("\n{}", "Available Secret Patterns:".cyan().bold());
            
            for (i, pattern) in scanner.patterns().iter().enumerate() {
                println!(
                    "{}. {} - {}",
                    (i + 1).to_string().white().bold(),
                    pattern.name.yellow(),
                    pattern.description.dimmed()
                );
            }
            println!("\n{} patterns available\n", scanner.patterns().len().to_string().green());
        }
    }
}

async fn parse_repo(repo: &str, github: &GitHubClient) -> (String, String) {
    if repo.contains('/') {
        let parts: Vec<&str> = repo.split('/').collect();
        (parts[0].to_string(), parts[1].to_string())
    } else {
        match github.list_repos().await {
            Ok(repos) => {
                if let Some(r) = repos.iter().find(|r| r.name == repo) {
                    let parts: Vec<&str> = r.full_name.split('/').collect();
                    (parts[0].to_string(), parts[1].to_string())
                } else {
                    eprintln!("{} Repository '{}' not found", "Error:".red().bold(), repo);
                    std::process::exit(1);
                }
            }
            Err(e) => {
                eprintln!("{} Failed to resolve repo: {}", "Error:".red().bold(), e);
                std::process::exit(1);
            }
        }
    }
}

async fn scan_repository(github: &GitHubClient, owner: &str, repo_name: &str, json: bool) {
    let full_name = format!("{}/{}", owner, repo_name);
    
    if !json {
        println!("Scanning {}...", full_name.cyan());
    }

    let scanner = Scanner::new();
    let mut all_findings = Vec::new();
    let mut total_files = 0;
    let total_branches;

    let branches = match github.list_branches(owner, repo_name).await {
        Ok(b) => b,
        Err(e) => {
            eprintln!("{} Failed to list branches: {}", "Error:".red().bold(), e);
            return;
        }
    };

    total_branches = branches.len();

    if !json {
        let branch_names: Vec<&str> = branches.iter().map(|b| b.name.as_str()).collect();
        println!("Branches ({}): {}", total_branches, branch_names.join(", ").yellow());
    }

    for branch in &branches {
        if !json {
            println!("  Scanning branch: {}...", branch.name.cyan());
        }

        match github.get_repo_files(owner, repo_name, "", Some(&branch.name)).await {
            Ok(files) => {
                total_files += files.len();
                for file_path in files {
                    match github.get_file_content(owner, repo_name, &file_path, Some(&branch.name)).await {
                        Ok(content) => {
                            let findings = scanner.scan_content(&file_path, &content);
                            for mut finding in findings {
                                finding.file_path = format!("[{}] {}", branch.name, finding.file_path);
                                all_findings.push(finding);
                            }
                        }
                        Err(_) => continue,
                    }
                }
            }
            Err(_) => continue,
        }
    }

    if json {
        print_findings_json(&all_findings);
    } else {
        print_findings(&all_findings, &full_name);
        println!("Branches scanned: {}", total_branches.to_string().cyan());
        print_scan_summary(1, total_files, all_findings.len());
    }
}