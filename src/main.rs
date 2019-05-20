use structopt::StructOpt;
use std::process::Command;

/// Search for a pattern in a file and display the lines that contain it.
/// Check out a pull request
#[derive(StructOpt)]
struct Cli {
    /// The pull request number to look for
    pull_number: i32,
}

fn main() {
    let args = Cli::from_args();
    let branch = format!("pull_{}", args.pull_number);
    let branch_to_pull = format!("refs/pull/{}/head:{}", args.pull_number, branch);
    let branch_output = Command::new("git").arg("branch-current").output().expect("Error fetching current branch");
    if branch_output.status.success() {
        let current_branch = String::from_utf8(branch_output.stdout).expect("Expected stdout to be utf8 vector");
        if current_branch.trim() == branch {
            println!("Already on branch {}", branch);
            return;
        }
    }
    let output = Command::new("git").arg("fetch").arg("upstream").arg(&branch_to_pull).output().expect("Error fetching branch");
    if output.status.success() {
        let stdout = String::from_utf8(output.stdout).expect("Expected stdout to be utf8 vector");
        if stdout.trim().len() > 0 {
            println!("{}", stdout);
        }
    }
    else {
        let output = Command::new("git").arg("fetch").arg("origin").arg(&branch_to_pull).status().expect("Error fetching branch");
        if !output.success() {
            return;
        }
    }
    Command::new("git").arg("checkout").arg(branch).status().expect("Failed to fetch branch");
}
