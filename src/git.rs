use std::process::Command;
use std::collections::HashMap;

#[derive(Debug)]
struct FileStats {
    file: String,
    lines_changed: u32,
    stats_graphical: String,
    raw_line: String,
}

#[derive(Debug)]
struct SummaryStats {
    files_changed: u32,
    insertions: u32,
    deletions: u32,
    raw_line: String,
}

#[derive(Debug)]
pub struct GitData {
    start_commit: String,
    end_commit: String,
    files: Vec<String>,
    file_stats: HashMap<String, FileStats>,
    summary_stats: SummaryStats,
}

impl GitData {
    pub fn new(start_commit: &str, end_commit: &str) -> Self {
        let mut git = GitData {
            start_commit: start_commit.to_string(),
            end_commit: end_commit.to_string(),
            files: Vec::new(),
            file_stats: HashMap::new(),
            summary_stats: SummaryStats {
                files_changed: 0,
                insertions: 0,
                deletions: 0,
                raw_line: String::new(),
            },
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

    fn extract_summary_stats(&mut self, summary_line: &str) {
        self.summary_stats.raw_line = summary_line.trim().to_string();

        let files_changed = summary_line.split_whitespace().nth(0);
        let insertions = summary_line.split_whitespace().nth(3);
        let deletions = summary_line.split_whitespace().nth(5);

        self.summary_stats.files_changed = files_changed.unwrap().parse().unwrap();
        self.summary_stats.insertions = insertions.unwrap().parse().unwrap();
        self.summary_stats.deletions = deletions.unwrap().parse().unwrap();
    }

    fn grab_diff_stats(&mut self) {
        let output = self.execute_git_diff(vec!["--stat"]);
        let mut lines = output.lines().rev();  // Reverse to grab summary first.
        
        self.extract_summary_stats(lines.next().unwrap());

        for line in lines {
            let trimmed_line = line.trim();
            let mut split_by_separator = trimmed_line.split(" | ");
            
            let file = split_by_separator.next().unwrap().trim();
            let mut stats_split = split_by_separator.next().unwrap().split_whitespace();
            
            self.file_stats.insert(file.to_string(), FileStats {
                file: file.to_string(),
                lines_changed: stats_split.next().unwrap().parse().unwrap(),
                stats_graphical: stats_split.nth(0).unwrap().to_string(),
                raw_line: trimmed_line.to_string(),
            });
        }
    }
}
