use crate::task::{Task, TaskSource, TaskStatus};
use chrono;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs;
use std::io::Write;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ObsidianConfig {
    pub notes_path: String,
    pub daily_notes: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObsidianHandler {
    pub config: ObsidianConfig,
    task_tag_map: std::collections::HashMap<TaskSource, String>,
    pub vault_path: String,
}

impl ObsidianHandler {
    pub fn new(config: ObsidianConfig) -> Self {
        let mut task_tag_map: std::collections::HashMap<TaskSource, String> =
            std::collections::HashMap::new();
        task_tag_map.insert(TaskSource::PullRequest, "#todo/work/pr".to_string());
        task_tag_map.insert(TaskSource::Issue, "#todo/work/tasks".to_string());
        task_tag_map.insert(TaskSource::JiraTicket, "#todo/work/jira".to_string());

        let vault_path = std::env::var("OBSIDIAN_VAULT_PATH").expect("OBSIDIAN_VAULT_PATH not set");

        ObsidianHandler {
            config,
            task_tag_map,
            vault_path,
        }
    }

    pub fn calculate_sha256(input: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(input);
        let result = hasher.finalize();
        format!("{:x}", result)
    }

    fn get_file_hashed_lines<P>(filename: P) -> io::Result<Vec<String>>
    where
        P: AsRef<Path>,
    {
        let file = fs::File::open(&filename)?;
        let reader = io::BufReader::new(file);
        Ok(reader
            .lines()
            .map(|l| ObsidianHandler::calculate_sha256(&l.unwrap()))
            .collect())
    }
}

pub trait HandleTask {
    fn today(&self) -> String {
        let now = chrono::Local::now();
        now.format("%Y-%m-%d").to_string()
    }

    fn add_tasks(&self, tasks: Vec<Task>);
}

impl HandleTask for ObsidianHandler {
    fn add_tasks(&self, tasks: Vec<Task>) {
        let today = self.today();
        let file_path: String;
        if self.config.daily_notes {
            file_path = format!("{}/{}/{}.md", self.vault_path, self.config.notes_path, today);
        } else {
            file_path = format!("{}/{}/tasks.md", self.vault_path, self.config.notes_path);
        }

        // create the file or fail if it exists
        let new_file = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&file_path);

        match new_file {
            Ok(_) => println!("Created file: {}", &file_path),
            _ => (),
        }

        let already_present_tasks = ObsidianHandler::get_file_hashed_lines(&file_path)
            .expect(format!("Could not read file: {}", &file_path).as_str());

        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open(&file_path)
            .expect(format!("Could not open file: {}", &file_path).as_str());

        for task in tasks {
            if task.status == TaskStatus::Done || task.status == TaskStatus::Review {
                continue;
            }

            let tag = self.task_tag_map.get(&task.source).unwrap();
            let task_string = format!("- [ ] {} {}", tag, task.to_string());
            let hashed_task = ObsidianHandler::calculate_sha256(&task_string.trim().to_string());
            if !already_present_tasks.contains(&hashed_task) {
                println!("Adding task: {}", task.name);
                file.write_all(task_string.as_bytes())
                    .expect("Could not write to file");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_sha256() {
        let input = "Hello, World!";
        let hashed = ObsidianHandler::calculate_sha256(input);
        assert_eq!(hashed, "dffd6021bb2bd5b0af676290809ec3a53191dd81c7f70a4b28688a362182986f");
    }
}
