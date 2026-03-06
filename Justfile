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

dev-db-setup env_file=".env":
    ./crates/db/dev_setup.sh {{env_file}}

## Run the code generators
wd:
    cargo watch -w ./crates/db/queries/ -s 'clorinde live -q ./crates/db/queries/ -d crates/db-gen && cargo fmt'

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
        -w crates/octo-ui \
        -w crates/octo-assets/dist \
        -w crates/tool-runtime \
        --no-gitignore -x "run --bin {{binary}}"

wo env_file=".env": (_watch "octo" env_file)

wa env_file=".env": (_watch "agent-runtime" env_file)

wci env_file=".env": (_watch "telegram-ingress-polling" env_file)

wce env_file=".env": (_watch "telegram-egress" env_file)

wtw:
    cd /workspace/crates/octo-assets && tailwind-extra -i ./input.css -o ./dist/tailwind.css --watch

wi:
    cargo watch \
      -w crates/octo-islands \
      -s 'cargo build -p octo-islands --target wasm32-unknown-unknown --release && \
          wasm-bindgen \
            target/wasm32-unknown-unknown/release/octo_islands.wasm \
            --target web \
            --out-dir crates/octo-assets/dist'



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
