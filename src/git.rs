use std::process::Command;

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

    fn execute_git_diff(&self, flags: Vec<&str>) -> String {
        let mut command = Command::new("git");
        command.arg("diff");
        command.arg(&self.start_commit);
        command.arg(&self.end_commit);

        for flag in flags {
            command.arg(flag);
        }

        let command_output = command.output().unwrap();
        String::from_utf8(command_output.stdout).unwrap()
    }

    fn grab_diff_files(&mut self) {
        let output = self.execute_git_diff(vec!["--name-only"]);
        self.files = output.lines().map(|s| s.to_string()).collect();
    }

    fn grab_diff_stats(&mut self) {
        let output = self.execute_git_diff(vec!["--stat"]);
        self.stats = output;
    }
}
