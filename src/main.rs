use chrono::prelude::*;
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf, process::Command};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Record a change
    #[command(alias = "a")]
    Add {
        /// Change description
        message: String,
        /// Type of change
        #[arg(short, long, default_value = "feature")]
        type_: String,
    },
    /// Commit and push changes
    #[command(alias = "c")]
    Commit {
        /// Target branch
        #[arg(short, long)]
        branch: Option<String>,
        /// Skip pushing changes
        #[arg(long)]
        no_push: bool,
    },
    /// List recorded changes
    #[command(alias = "l")]
    List,
}

#[derive(Debug, Serialize, Deserialize)]
struct Change {
    timestamp: String,
    type_: String,
    description: String,
    files: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    default_branch: String,
    commit_templates: std::collections::HashMap<String, String>,
    auto_push: bool,
}

struct GitTracker {
    config: Config,
    changes: Vec<Change>,
    // config_path: PathBuf,
    changes_path: PathBuf,
}

impl GitTracker {
    fn new() -> Self {
        let config_path = PathBuf::from(".gt-config.json");
        let changes_path = PathBuf::from(".gt-changes.json");

        let config = if config_path.exists() {
            serde_json::from_str(&fs::read_to_string(&config_path).unwrap()).unwrap()
        } else {
            let mut templates = std::collections::HashMap::new();
            templates.insert("feature".to_string(), "feat: {message}".to_string());
            templates.insert("fix".to_string(), "fix: {message}".to_string());
            templates.insert("docs".to_string(), "docs: {message}".to_string());
            templates.insert("style".to_string(), "style: {message}".to_string());
            templates.insert("refactor".to_string(), "refactor: {message}".to_string());
            templates.insert("test".to_string(), "test: {message}".to_string());
            templates.insert("chore".to_string(), "chore: {message}".to_string());

            let config = Config {
                default_branch: "main".to_string(),
                commit_templates: templates,
                auto_push: true,
            };
            fs::write(&config_path, serde_json::to_string_pretty(&config).unwrap()).unwrap();
            config
        };

        let changes = if changes_path.exists() {
            serde_json::from_str(&fs::read_to_string(&changes_path).unwrap()).unwrap()
        } else {
            Vec::new()
        };

        Self {
            config,
            changes,
            // config_path,
            changes_path,
        }
    }

    fn save_changes(&self) {
        fs::write(
            &self.changes_path,
            serde_json::to_string_pretty(&self.changes).unwrap(),
        )
        .unwrap();
    }

    fn get_modified_files(&self) -> Vec<String> {
        let output = Command::new("git")
            .arg("diff")
            .arg("--name-only")
            .output()
            .expect("Failed to execute git command");

        String::from_utf8_lossy(&output.stdout)
            .lines()
            .map(String::from)
            .filter(|s| !s.is_empty())
            .collect()
    }

    fn get_current_branch(&self) -> Option<String> {
        let output = Command::new("git")
            .args(["branch", "--show-current"])
            .output()
            .ok()?;

        Some(
            String::from_utf8_lossy(&output.stdout)
                .trim()
                .to_string(),
        )
    }

    fn add_change(&mut self, description: String, type_: String) {
        let timestamp = Local::now().to_rfc3339();
        let files = self.get_modified_files();

        self.changes.push(Change {
            timestamp,
            type_: type_.clone(),
            description: description.clone(),
            files: files.clone(),
        });

        self.save_changes();
        println!("✓ Recorded {}: {}", type_, description);
        if !files.is_empty() {
            println!("  Modified files: {}", files.join(", "));
        }
    }

    fn generate_commit_message(&self) -> String {
        if self.changes.is_empty() {
            return "No changes recorded".to_string();
        }

        self.changes
            .iter()
            .map(|change| {
                let template = self
                    .config
                    .commit_templates
                    .get(&change.type_)
                    .unwrap_or(&self.config.commit_templates["feature"])
                    .replace("{message}", &change.description);

                if change.files.is_empty() {
                    template
                } else {
                    format!("{}\n\n{}", template, change.files.iter().map(|f| format!("- {}", f)).collect::<Vec<_>>().join("\n"))
                }
            })
            .collect::<Vec<_>>()
            .join("\n\n")
    }

    fn commit_and_push(&mut self, branch: Option<String>, no_push: bool) -> Result<(), Box<dyn std::error::Error>> {
        if self.changes.is_empty() {
            println!("No changes to commit");
            return Ok(());
        }
    
        // Verify we're in a git repository
        if !Command::new("git")
            .args(["rev-parse", "--git-dir"])
            .status()?
            .success()
        {
            println!("❌ Not in a git repository");
            return Ok(());
        }
    
        // Check if there are any git changes to commit
        let status_output = Command::new("git")
            .args(["status", "--porcelain"])
            .output()?;
    
        if status_output.stdout.is_empty() {
            println!("No git changes detected to commit");
            return Ok(());
        }
    
        // Determine target branch
        let branch = branch.or_else(|| self.get_current_branch()).unwrap_or_else(|| self.config.default_branch.clone());
    
        // Stage changes
        println!("Staging changes...");
        if !Command::new("git").arg("add").arg(".").status()?.success() {
            println!("❌ Failed to stage changes");
            return Ok(());
        }
    
        // Verify files were staged
        let staged_output = Command::new("git")
            .args(["diff", "--cached", "--quiet"])
            .status()?;
    
        if staged_output.success() {
            println!("❌ No changes were staged");
            return Ok(());
        }
    
        // Generate and verify commit message
        let commit_message = self.generate_commit_message();
        if commit_message.is_empty() {
            println!("❌ Empty commit message, nothing to commit");
            return Ok(());
        }
    
        // Commit changes
        println!("Committing changes...");
        if !Command::new("git")
            .arg("commit")
            .arg("-m")
            .arg(&commit_message)
            .status()?
            .success()
        {
            println!("❌ Failed to commit changes");
            return Ok(());
        }
    
        // Push if enabled
        if !no_push && self.config.auto_push {
            println!("Pushing to remote...");
            
            // Check if remote exists
            if !Command::new("git")
                .args(["remote", "get-url", "origin"])
                .status()?
                .success()
            {
                println!("❌ Remote 'origin' not found");
                println!("✓ Changes committed successfully (push skipped - no remote)");
                self.changes.clear();
                self.save_changes();
                return Ok(());
            }
    
            // Determine push arguments based on remote branch existence
            let remote_branch_exists = Command::new("git")
                .args(["ls-remote", "--heads", "origin", &branch])
                .output()?
                .stdout
                .len() > 0;
    
            let push_args = if !remote_branch_exists {
                println!("Creating new remote branch '{}'...", branch);
                vec!["push", "-u", "origin", &branch]
            } else {
                vec!["push", "origin", &branch]
            };
    
            if Command::new("git").args(&push_args).status()?.success() {
                println!("✓ Successfully pushed changes to {}", branch);
            } else {
                println!("❌ Failed to push changes to remote");
                println!("  Your commits are saved locally. To push later, run:");
                println!("  git push origin {}", branch);
                return Ok(());
            }
        } else {
            println!("✓ Successfully committed changes (push skipped)");
        }
    
        // Clear tracked changes only after successful operations
        self.changes.clear();
        self.save_changes();
    
        Ok(())
    }
    
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let mut tracker = GitTracker::new();

    match cli.command {
        Commands::Add { message, type_ } => {
            tracker.add_change(message, type_);
        }
        Commands::Commit { branch, no_push } => {
            tracker.commit_and_push(branch, no_push)?;
        }
        Commands::List => {
            if tracker.changes.is_empty() {
                println!("No changes recorded yet");
            } else {
                println!("\nRecorded changes:");
                for (i, change) in tracker.changes.iter().enumerate() {
                    println!(
                        "{}. [{}] {}: {}",
                        i + 1,
                        change.timestamp,
                        change.type_,
                        change.description
                    );
                    if !change.files.is_empty() {
                        println!("   Files: {}", change.files.join(", "));
                    }
                }
            }
        }
    }

    Ok(())
}