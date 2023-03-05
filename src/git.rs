#[derive(Debug)]
pub struct GitData {
    start_commit: String,
    end_commit: String,
    files: Vec<String>,
    stats: String,
}

impl GitData {
    pub fn new(start_commit: &str, end_commit: &str) -> Self {
        let mut git = GitData {
            start_commit: start_commit.to_string(),
            end_commit: end_commit.to_string(),
            files: Vec::new(),
            stats: String::new(),
        };
        git.grab_diff_files();
        git.grab_diff_stats();
        git
    }

    fn grab_diff_files(&mut self) {
        let output = std::process::Command::new("git")
            .arg("diff")
            .arg(&self.start_commit)
            .arg(&self.end_commit)
            .arg("--name-only")
            .output()
            .expect("Failed to execute `git diff` command.");

        let output = String::from_utf8(output.stdout).unwrap();
        self.files = output.lines().map(|s| s.to_string()).collect();
    }

    fn grab_diff_stats(&mut self) {
        let output = std::process::Command::new("git")
            .arg("diff")
            .arg(&self.start_commit)
            .arg(&self.end_commit)
            .arg("--stat")
            .output()
            .expect("Failed to execute `git diff` command.");

        let output = String::from_utf8(output.stdout).unwrap();
        self.stats = output;
    }
}
