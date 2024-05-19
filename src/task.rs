use serde::{Deserialize, Serialize};

pub struct Task {
    pub name: String,
    pub source: TaskSource,
    pub status: TaskStatus,
}

#[derive(Hash, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum TaskSource {
    Issue,
    PullRequest,
    JiraTicket
}

#[derive(PartialEq)]
pub enum TaskStatus {
    Todo,
    SelectedForDevelopment,
    InProgress,
    Blocked,
    Review,
    Done,
    Open,
}

use std::fmt;

impl fmt::Display for TaskSource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TaskSource::Issue => write!(f, "Issue"),
            TaskSource::PullRequest => write!(f, "PR"),
            TaskSource::JiraTicket => write!(f, "Jira"),
        }
    }
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TaskStatus::Todo => write!(f, "TODO"),
            TaskStatus::SelectedForDevelopment => write!(f, "Selected For Development"),
            TaskStatus::Open => write!(f, "Open"),
            TaskStatus::InProgress => write!(f, "In Progress"),
            TaskStatus::Blocked => write!(f, "Blocked"),
            TaskStatus::Review => write!(f, "Review"),
            TaskStatus::Done => write!(f, "Done"),
        }
    }
}

impl Task {
    pub fn to_string(&self) -> String {
        format!("{}: {} - {}\n", self.source, self.name, self.status)
    }
}
