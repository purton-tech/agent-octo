## Agent Octo

<p align="center">
    <picture>
        <source media="(prefers-color-scheme: light)" srcset=".github/octo.png">
        <img src=".github/octo.png" alt="Agent Octo" width="250">
    </picture>
</p>

An [OpenClaw](https://openclaw.ai/) clone with **90% reduction** in token usage.

1. The Token Usage is around 10% of OpenClaw whilst still supporting most of the main features.
1. Runs the [Monty](https://github.com/pydantic/monty) Python interpreter to give the LLM sandboxed scripting abilities.
1. Yaml plugins which are [Open API](https://www.openapis.org/) specifications allow you to add tools at runtime with no code.
1. All Plugins are discoverable. Only a few tokens are used in the System Prompt and no entries in the Tool Definitions.
1. Uses [Postgresql](https://www.postgresql.org/) for storage and backup.
1. Kubernetes deployments with [Stack](https://stack-cli.com/)
1. `docker-compose.yaml` file for easy install and de-install.
1. Runs in minimal docker containers, `FROM scratch`.

Built with the [Rust on Nails](https://rust-on-nails.com/) architecture for secure full stack web applications.

## Installation

Download the `docker-compose.yaml` file.

```sh
curl -LO https://raw.githubusercontent.com/purton-tech/agent-octo/main/infra-as-code/docker-compose.yaml
```

Create a `.env` file and add your Telegram Token and model provider details.

```
TELEGRAM_BOT_TOKEN=<telegram-token>
OPENAI_API_KEY=<api key>
```

Run the server

```sh
docker-compose up
```