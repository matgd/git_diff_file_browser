// git diff <commit> <commit> --name-only
// ex. git diff HEAD~1 --name-only
// ex. git diff HEAD~1 HEAD~2 --name-only
//
// git diff <commit> <commit> --stat
// ??
//
// git diff <commit> <commit> <file>
// ex. git diff HEAD~1 HEAD~2 main.rs

use clap::Parser;

mod git;

#[derive(Parser)]
struct Cli {
    start_commit: String,
    end_commit: String,
}

fn main() {
    let args = Cli::parse();

    let git = git::GitData::new(args.start_commit.as_str(), args.end_commit.as_str());
    println!("{:?}", git);
}
