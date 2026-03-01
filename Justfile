set dotenv-load := true

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
    stack secrets --manifest infra-as-code/stack.yaml --db-host host.docker.internal --db-port 30061 >> .env
    sed -i 's/^MIGRATIONS_URL=/DATABASE_URL=/' .env

wc:
    cargo watch -w ./crates/db/queries/ -s 'clorinde live -q ./crates/db/queries/ -d crates/clorinde $DATABASE_URL'

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
