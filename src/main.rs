use clap::{Parser, Subcommand};
use owo_colors::OwoColorize;
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
    Remove { branch: String },
    New { branch: String },
}

#[derive(Debug)]
struct Worktree {
    path: String,
    branch: String,
}

fn git_output(args: &[&str]) -> String {
    let output = Command::new("git")
        .args(args)
        .output()
        .expect("failed to run git");

    if !output.status.success() {
        eprintln!("{}", String::from_utf8_lossy(&output.stderr).red());
        std::process::exit(output.status.code().unwrap_or(1));
    }

    String::from_utf8_lossy(&output.stdout).to_string()
}
fn branch_to_dir_name(branch: &str) -> String {
    branch.replace("/", "-")
}

fn list_worktrees() -> Vec<Worktree> {
    let output = Command::new("git")
        .args(["worktree", "list", "--porcelain"])
        .output()
        .expect("failed to run git");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut worktrees = Vec::new();

    for block in stdout.split("\n\n") {
        let mut path = None;
        let mut branch = None;

        for line in block.lines() {
            if let Some(p) = line.strip_prefix("worktree ") {
                path = Some(p.to_string());
            }
            if let Some(name) = line.strip_prefix("branch refs/heads/") {
                branch = Some(name.to_string());
            }
        }

        if let (Some(path), Some(branch)) = (path, branch) {
            worktrees.push(Worktree { path, branch });
        }
    }

    worktrees
}

fn print_worktrees() {
    let worktrees = list_worktrees();

    if worktrees.is_empty() {
        println!("{}", "No worktrees found.".yellow());
    } else {
        println!("{}", "Worktrees:".green().bold());
        for worktree in worktrees {
            println!(
                "  {} {} {}",
                worktree.branch.cyan().bold(),
                "->".dimmed(),
                worktree.path.dimmed(),
            );
        }
    }
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
        Commands::List => {
            print_worktrees();
            ExitCode::SUCCESS
        }
        Commands::Add { path, branch } => run_git(&["worktree", "add", &path, &branch]),
        Commands::Remove { branch } => {
            let dir_name = branch_to_dir_name(&branch);
            let path = format!("worktrees/{}", dir_name);
            run_git(&["worktree", "remove", &path])
        }
        Commands::New { branch } => {
            let dir_name = branch_to_dir_name(&branch);
            let path = format!("worktrees/{}", dir_name);
            run_git(&["worktree", "add", &path, "-b", &branch])
        }
    }
}
