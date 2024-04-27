use super::Fetch;
use crate::task;
use reqwest;
use serde_json;

#[derive(Debug)]
pub struct JiraSource {
    user: String,
    url: String,
    token: String,
}

impl JiraSource {
    pub fn new() -> Self {
        let user = std::env::var("JIRA_USER").expect("JIRA_USER not found");
        let token = std::env::var("JIRA_TOKEN").expect("JIRA_TOKEN not found");
        let url = std::env::var("JIRA_URL").expect("JIRA_URL not found");
        
        JiraSource { user, url, token }
    }
}


impl Fetch<task::Task> for JiraSource {
    async fn fetch(&self, _filter_my_task: bool) -> Vec<task::Task> {
        let client = reqwest::Client::new();
        let response = client
            .get(&format!("{}/rest/api/2/search?{}", self.url, r#"jql=assignee=currentuser() AND status!=Done"#))
            .basic_auth(&self.user, Some(&self.token))
            .header("Accept", "application/json")
            .send()
            .await
            .expect("Could not fetch Jira tasks");

        let body = response.text().await.expect("Could not read response body");
        let jira_tasks: serde_json::Value = serde_json::from_str(&body).expect("Could not parse JSON");

        let mut tasks = vec![];
        for issue in jira_tasks["issues"].as_array().expect("Could not find issues") {
            let key = issue["key"].as_str().expect("Could not find key");
            let issue_url  = format!("{}/browse/{}", self.url, key);
            let parent_issue = issue["fields"]["parent"]["fields"]["summary"].as_str().unwrap_or("No parent");
            let issue_summary = issue["fields"]["summary"].as_str().unwrap_or("No summary");

            let task = task::Task {
                name: format!("<a href={}>{}</a>: {} - {}", issue_url, key, issue_summary, parent_issue),
                source: task::TaskSource::JiraTicket,
                status: task::TaskStatus::Open,
            };
            tasks.push(task);
        }

        tasks
    }
}

#[cfg(test)]
mod tests {
    use dotenv::dotenv;
    use super::*;

    #[test]
    fn instantiation_test() {
        dotenv().ok();
        let jira = JiraSource::new();
        assert!(!jira.user.is_empty());
        assert!(!jira.url.is_empty());
        assert!(!jira.token.is_empty());
    }

    #[tokio::test]
    async fn fetch_test() {
        dotenv().ok();
        let jira = JiraSource::new();
        let tasks = jira.fetch(false).await;
        assert!(!tasks.is_empty());
    }
}
