use structopt::StructOpt;
use std::process::Command;

/// Search for a pattern in a file and display the lines that contain it.
/// Check out a pull request
#[derive(StructOpt)]
struct Cli {
    /// The pull request number to look for
    pull_number: i32,

    /// What to name the new branch
    #[structopt(short = "b", long = "branch")]
    branch: Option<String>,
}

fn main() {
    let args = Cli::from_args();
    let branch = match args.branch {
        Some(b) => b,
        None => format!("pull_{}", args.pull_number)
    };
    let base_branch_to_pull = format!("pull/{}/head", args.pull_number);
    let branch_to_pull = format!("{}:{}", base_branch_to_pull, branch);
    let branch_output = Command::new("git").arg("branch-current").output().expect("Error fetching current branch");
    if branch_output.status.success() {
        let current_branch = String::from_utf8(branch_output.stdout).expect("Expected stdout to be utf8 vector");
        if current_branch.trim() == branch {
            println!("Already on branch {}. Pulling most recent changes...", branch);
            get_branch("fetch", &base_branch_to_pull);
            get_branch("pull", &base_branch_to_pull);
            return;
        }
    }
    get_branch("fetch", &branch_to_pull);
    Command::new("git").arg("checkout").arg(branch).status().expect("Failed to fetch branch");
}


fn get_branch(action: &str, branch: &String) {
    let output = Command::new("git").arg(action).arg("upstream").arg(&branch).output().expect("Error fetching branch");
    if output.status.success() {
        let stdout = String::from_utf8(output.stdout).expect("Expected stdout to be utf8 vector");
        if stdout.trim().len() > 0 {
            println!("{}", stdout);
        }
    }
    else {
        Command::new("git").arg(action).arg("origin").arg(&branch).output().expect("Error fetching branch");
    }
}
