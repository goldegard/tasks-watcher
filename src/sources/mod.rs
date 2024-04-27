pub mod github;
pub mod jira;

pub trait Fetch<Task> {
    async fn fetch(&self, filter_my_task: bool) -> Vec<Task>;
}
