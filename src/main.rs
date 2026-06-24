use clap::{Parser, Subcommand};
use std::process::{Command, ExitCode};

#[derive(Parser)]
#[command(name = "sii")]
#[command(about = "A tiny git worktree helper", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    List,
    Add { path: String, branch: String },
    Remove { path: String },
    New { branch: String },
}

fn branch_to_dir_name(branch: &str) -> String {
    branch.replace("/", "-")
}

fn run_git(args: &[&str]) -> ExitCode {
    let status = Command::new("git")
        .args(args)
        .status()
        .expect("Failed to execute git command");

    if status.success() {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(status.code().unwrap_or(1) as u8)
    }
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    match cli.command {
        Commands::List => run_git(&["worktree", "list"]),
        Commands::Add { path, branch } => run_git(&["worktree", "add", &path, &branch]),
        Commands::Remove { path } => run_git(&["worktree", "remove", &path]),
        Commands::New { branch } => {
            let dir_name = branch_to_dir_name(&branch);
            let path = format!("worktrees/{}", dir_name);
            run_git(&["worktree", "add", &path, "-b", &branch])
        }
    }
}
