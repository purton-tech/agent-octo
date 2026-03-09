## Agent Octo

<p align="center">
    <picture>
        <source media="(prefers-color-scheme: light)" srcset=".github/octo.png">
        <img src=".github/octo.png" alt="Agent Octo" width="250">
    </picture>
</p>

Inspired by [OpenClaw](https://openclaw.ai/), Agent Octo adds some practical features:

1. Agent Octo is multi-user and multi-tenant, so you can run it for your team, family, or as your own SaaS.
1. It includes a multi-threaded, async Python sandbox written in Rust for AI workloads, powered by [Monty](https://github.com/pydantic/monty).
1. The sandbox supports [Code Mode](https://blog.cloudflare.com/code-mode-mcp/) to reduce token usage.
1. Agent Octo uses [OpenAPI](https://www.openapis.org/) (Swagger) specs, letting you add integrations at runtime.
1. Tools are discoverable and lightweight in context; in the Tool Definition section, the only tool is `exec_python`.
1. Agent Octo supports OAuth2 for configuring API integrations (for example, Google and Microsoft).

Built with the [Rust on Nails](https://rust-on-nails.com/) architecture for secure full stack web applications.

## Installation

Download the `docker-compose.yaml` file.

```sh
curl -LO https://raw.githubusercontent.com/purton-tech/agent-octo/main/infra-as-code/docker-compose.yaml
```

then

```sh
docker compose up
```

Then open `http://localhost:3000`.
