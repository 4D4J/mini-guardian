use crate::scanner::Finding;
use colored::*;

pub fn print_findings(findings: &[Finding], repo_name: &str) {
    if findings.is_empty() {
        println!("{} No secrets found in {}", "✓".green().bold(), repo_name.cyan());
        return;
    }

    println!(
        "\nFound {} potential secret(s) in {}:\n",
        findings.len().to_string().red().bold(),
        repo_name.cyan()
    );

    for (i, finding) in findings.iter().enumerate() {
        println!(
            "{}. {} [{}]",
            (i + 1).to_string().white().bold(),
            finding.secret_type.red().bold(),
            finding.file_path.blue()
        );
        println!(
            "   Line {}: {}",
            finding.line_number.to_string().yellow(),
            truncate_line(&finding.line_content, 80).dimmed()
        );
        println!(
            "   Match: {}",
            finding.masked_text().red()
        );
        println!();
    }
}

pub fn print_findings_json(findings: &[Finding]) {
    let json = serde_json::to_string_pretty(findings).unwrap_or_else(|_| "[]".to_string());
    println!("{}", json);
}

pub fn print_scan_summary(total_repos: usize, total_files: usize, total_findings: usize) {
    println!("{}", "SCAN SUMMARY".white().bold());
    println!("  Repositories scanned: {}", total_repos.to_string().cyan());
    println!("  Files scanned:        {}", total_files.to_string().cyan());
    
    if total_findings > 0 {
        println!(
            "  Secrets found:        {}",
            total_findings.to_string().red().bold()
        );
    } else {
        println!(
            "  Secrets found:        {}",
            "0".green().bold()
        );
    }
}

fn truncate_line(line: &str, max_len: usize) -> String {
    let trimmed = line.trim();
    if trimmed.len() > max_len {
        format!("{}...", &trimmed[..max_len])
    } else {
        trimmed.to_string()
    }
}
