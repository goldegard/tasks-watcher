# Task Watcher

Simple Rust utility to poll tasks and save them in your Obsidian daily task note.

In your daily life as a developer, you might have tasks assigned to you in different platforms like GitHub, Jira, etc. This utility will help you to keep track of all your tasks in one place.

At the moment it supports GitHub and Jira as task sources and Obsidian as the destination.

## Env variables
Set the following environment variables or create a `.env` file in the root of the project with the following content:
- `GITHUB_TOKEN`: GitHub token to access the GitHub API
- `OBSIDIAN_VAULT`: Path to your obisidian vault
- `FILTER_MY_TASK`: Filter only tasks assigned to you
- `JIRA_USER`: Jira user email
- `JIRA_TOKEN`: Jira API token
- `JIRA_URL`: Url of your Jira instance

## Configuration file
To setup the rest of the configuration, you'll need to create a `yaml` config file,
containing fields that will be parsed to populate the config object.

## Usage
```bash
cd task-watcher
cargo run
```

Use: `task-watcher --help` to see all available options.
