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
    let output = Command::new("git").arg("fetch").arg("upstream").arg(&branch_to_pull).output().expect("Error fetching branch");
    if output.status.success() {
        println!("{}", String::from_utf8(output.stdout).expect("Expected stdout to be utf8 vector"));
    }
    else {
        let output = Command::new("git").arg("fetch").arg("origin").arg(&branch_to_pull).status().expect("Error fetching branch");
        if !output.success() {
            return;
        }
    }
    Command::new("git").arg("checkout").arg(branch).status().expect("Failed to fetch branch");
}
