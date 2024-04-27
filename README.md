# Tasks Watcher

Simple Rust utility to poll tasks and save them in your Obsidian daily task note.

In your daily life as a developer, you might have tasks assigned to you in different platforms like GitHub, Jira, etc. This utility will help you to keep track of all your tasks in one place.

At the moment it supports GitHub and Jira as task sources and Obsidian as the destination.

## Env variables
Set the following environment variables or create a `.env` file in the root of the project with the following content:
- `GITHUB_TOKEN`: GitHub token to access the GitHub API
- `OBSIDIAN_VAULT_PATH`: Path to your obisidian vault
- `FILTER_MY_TASK`: Filter only tasks assigned to you
- `JIRA_USER`: Jira user email
- `JIRA_TOKEN`: Jira API token
- `JIRA_URL`: Url of your Jira instance

## Configuration file
You'll need to create a `yaml` config file,
containing fields that will be parsed to populate the config object. Look at the [Config](src/config.rs) struct to see all available fields.

## Usage
```bash
cd tasks-watcher
cargo run -- -e <your-env>.env -c <your-config-file>.yaml
```

Use: `tasks-watcher --help` to see all available options.

## System-wide installation
First install `cargo-deb` to create the deb package:
`cargo install cargo-deb`

Then run the following command to create the deb package and install it in your system:
```bash
cargo build --release && cargo deb
sudo dpkg -i target/debian/tasks-watcher_0.1.0-1_amd64.deb
```

## Systemd service
This utility can be used as a systemd service, unfortunately at the present moment `cargo deb` doesn't support user systemd service. This is needed here, because the service is supposed to run as a user service, using per-user configuration files.

To enable the systemd service: move the `task-watcher.service` file to `/etc/systemd/user/`, and run the following commands:
```bash
systemctl --user daemon-reload
systemctl --user enable tasks-watcher
systemctl --user start tasks-watcher
```
This will start the service and by default it will look for `config.yaml` and `env.env` in `~/.config/task-watcher/`.

## TODOs
- [ ] Use of `cargo deb` to install the service: see [service-example](https://github.com/vasilakisfil/hello.service/tree/master) and [cargo-deb](https://github.com/kornelski/cargo-deb)
