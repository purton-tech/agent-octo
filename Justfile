set dotenv-load := true

## Manage Our k3d clsuter
dev-init:
    k3d cluster delete k3d-octo
    k3d cluster create k3d-octo --agents 1 -p "30060-30066:30060-30066@agent:0"
    just get-config

dev-setup:
    stack init
    stack deploy --manifest infra-as-code/stack.yaml --profile dev

dev-secrets:
    touch .env
    printf "\n" >> .env
    stack secrets --manifest infra-as-code/stack.yaml --profile dev --db-host host.docker.internal --db-port 30061 >> .env
    sed -i 's/^MIGRATIONS_URL=/DATABASE_URL=/' .env

runtime-secrets env_file=".env":
    #!/usr/bin/env bash
    set -euo pipefail

    if [ ! -f "{{env_file}}" ]; then
        echo "Missing env file: {{env_file}}" >&2
        exit 1
    fi

    set -a
    . "{{env_file}}"
    set +a

    : "${TELEGRAM_BOT_TOKEN:?TELEGRAM_BOT_TOKEN not set in {{env_file}}}"
    : "${OPENAI_API_KEY:?OPENAI_API_KEY not set in {{env_file}}}"

    kubectl create secret generic octo-runtime \
        --namespace agent-octo \
        --from-literal=telegram-bot-token="${TELEGRAM_BOT_TOKEN}" \
        --from-literal=openai-api-key="${OPENAI_API_KEY}" \
        --dry-run=client \
        -o yaml \
        | kubectl apply -f -

## Run the code generators
wc:
    cargo watch -w ./crates/db/queries/ -s 'clorinde live -q ./crates/db/queries/ -d crates/db-gen'

_watch binary env_file=".env":
    #!/usr/bin/env bash
    set -euo pipefail

    if [ ! -f "{{env_file}}" ]; then
        echo "Missing env file: {{env_file}}  run just dot-env" >&2
        exit 1
    fi

    set -a
    . "{{env_file}}"
    set +a

    mold -run cargo watch \
        --workdir /workspace/ \
        -w crates/agent-runtime \
        -w crates/channels \
        -w crates/db \
        -w crates/db-gen \
        -w crates/octo \
        -w crates/tool-runtime \
        --no-gitignore -x "run --bin {{binary}}"

wo env_file=".env": (_watch "octo" env_file)

wa env_file=".env": (_watch "agent-runtime" env_file)

wt env_file=".env": (_watch "channels" env_file)

# Retrieve the cluster kube config - so kubectl and k9s work.
get-config:
    k3d kubeconfig write k3d-octo --kubeconfig-merge-default
    sed -i "s/127\.0\.0\.1/host.docker.internal/g; s/0\.0\.0\.0/host.docker.internal/g" "$HOME/.kube/config"
    # Disable TLS verification for local dev
    sed -i '/certificate-authority-data/d' "$HOME/.kube/config"
    sed -i '/cluster:/a \ \ \ \ insecure-skip-tls-verify: true' "$HOME/.kube/config"
    echo "✅ kubeconfig updated and TLS verification disabled"

codex: 
    sudo npm install -g @openai/codex

octo:
    cargo binstall --no-confirm zellij
    zellij -l .devcontainer/layout.kdl
