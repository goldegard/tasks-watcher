mod config;
use config::Config;

mod obsidian_handler;
mod task;
use obsidian_handler::HandleTask;

mod sources;
use sources::Fetch;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // Path to the .env file
    #[clap(short, long)]
    env_file: Option<String>,

    // Path to the configuration yaml file
    #[clap(short, long)]
    config_file: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    dotenv::from_filename(args.env_file.unwrap_or(".env".to_string())).ok();


    let filter_my_task: bool = match std::env::var("FILTER_MY_TASK") {
        Ok(val) => val == "true",
        Err(_) => false,
    };

    loop {
        // read config and reset tasks
        let config = Config::from_file(&args.config_file);

        // instantiate sources
        let gh_source = sources::github::GitHubSource::new(
            config.github_config.current_user,
            config.github_config.repos,
        );
        let jira_source = sources::jira::JiraSource::new();

        let obsidian_handler = obsidian_handler::ObsidianHandler::new(
            config.obsidian_handler.notes_path, config.obsidian_handler.daily_notes,
        );


        //rerieve tasks
        let gh_tasks = gh_source.fetch(filter_my_task).await;
        let jira_tasks = jira_source.fetch(filter_my_task).await;

        // update tasks
        obsidian_handler.add_tasks(gh_tasks);
        obsidian_handler.add_tasks(jira_tasks);

        // sleep
        tokio::time::sleep(tokio::time::Duration::from_secs(60 * config.poll_interval)).await;
    }
}
