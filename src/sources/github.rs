use super::Fetch;
use crate::task;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Repo {
    pub user: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GitHubSource {
    pub current_user: String,
    pub repos: Vec<Repo>,
}

impl GitHubSource {
    pub fn new(my_user: String, repos: Vec<Repo>) -> Self {
        GitHubSource { current_user: my_user, repos }
    }

    fn get_token() -> String {
        std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN not found")
    }
}

impl Fetch<task::Task> for GitHubSource {
    async fn fetch(&self, filter_my_task: bool) -> Vec<task::Task> {
        let client = octocrab::Octocrab::builder()
            .personal_token(GitHubSource::get_token())
            .build()
            .expect("Could not create client");
        let mut tasks = vec![];
        
        // get PRs in the selected repos
        for repo in &self.repos {
            let prs = client
                .pulls(&repo.user, &repo.name)
                .list()
                .send()
                .await
                .unwrap();
            for pr in prs {
                let reviewers_names = match pr.requested_reviewers {
                    Some(reviewers) => reviewers
                        .iter()
                        .map(|r| r.login.clone())
                        .collect::<Vec<String>>(),
                    None => continue,
                };

                // filter only my PRs
                if filter_my_task && !reviewers_names.contains(&self.current_user) {
                    continue;
                }

                let pr_name = format!(
                    "<a href={}>#{}</a> {}",
                    pr.html_url
                        .expect(format!("PR #{} URL not found", pr.number).as_str()),
                    pr.number.to_string(),
                    pr.title.unwrap_or("".to_string())
                );

                let task = task::Task {
                    name: pr_name,
                    source: task::TaskSource::PullRequest,
                    status: task::TaskStatus::Open,
                };
                tasks.push(task);
            }
        }
        return tasks;
    }
}
