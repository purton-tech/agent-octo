#!/usr/bin/env bash
set -euo pipefail

ROOT="$(pwd)"
SESSION=octo

in_devcontainer() {
  [ -n "${REMOTE_CONTAINERS:-}" ] || [ -n "${CODESPACES:-}" ] || [ -f "/.dockerenv" ]
}

start_tmux() {

  command -v tmux >/dev/null 2>&1 || (sudo apt update && sudo apt install -y tmux)

  if ! command -v gitui >/dev/null 2>&1; then
    curl -L https://github.com/gitui-org/gitui/releases/download/v0.28.0/gitui-linux-x86_64.tar.gz \
      | sudo tar -xz -C /usr/local/bin --wildcards --strip-components=1 '*/gitui'
  fi

  if ! command -v hx >/dev/null 2>&1; then
    curl -L https://github.com/helix-editor/helix/releases/download/25.07.1/helix-25.07.1-x86_64-linux.tar.xz | sudo tar -xJ -C /opt \
      && sudo ln -sf /opt/helix-25.07.1-x86_64-linux/hx /usr/local/bin/hx
  fi

  cat > ~/.tmux.conf <<'EOF'
set -g status on
set -g status-position bottom
set -g pane-border-status top
set -g pane-border-format "#{pane_title}"
set-option -g mouse on
bind -n F1 select-window -t 0
EOF

  if tmux has-session -t "$SESSION" 2>/dev/null; then
    exec tmux attach -t "$SESSION"
  fi

  tmux -f ~/.tmux.conf new-session -d -s "$SESSION" -n dev -c "$ROOT"

  tmux select-pane -t "$SESSION:0.0" -T "shell"

  tmux split-window -h -t "$SESSION:0" -c "$ROOT"
  tmux select-pane -t "$SESSION:0.1" -T "octo"
  tmux send-keys -t "$SESSION:0.1" "just wo" C-m

  tmux split-window -v -t "$SESSION:0.1" -c "$ROOT"
  tmux select-pane -t "$SESSION:0.2" -T "db queries"
  tmux send-keys -t "$SESSION:0.2" "just wd" C-m

  tmux split-window -v -t "$SESSION:0.2" -c "$ROOT"
  tmux select-pane -t "$SESSION:0.3" -T "wasm"
  tmux send-keys -t "$SESSION:0.3" "just wi" C-m

  tmux split-window -v -t "$SESSION:0.3" -c "$ROOT"
  tmux select-pane -t "$SESSION:0.4" -T "tailwind"
  tmux send-keys -t "$SESSION:0.4" "just wtw" C-m

  tmux select-pane -t "$SESSION:0.0"
  tmux select-layout -t "$SESSION:0" main-vertical

  tmux select-window -t "$SESSION:0"
  exec tmux attach -t "$SESSION"
}

if in_devcontainer; then
  start_tmux
else
  command -v devcontainer >/dev/null 2>&1 || {
    echo "devcontainer CLI is not installed."
    echo "Install it with: npm install -g @devcontainers/cli"
    exit 1
  }

  devcontainer up --workspace-folder "$ROOT" >/dev/null
  exec devcontainer exec --workspace-folder "$ROOT" bash -lc "./octo-tmux.sh"
fi
