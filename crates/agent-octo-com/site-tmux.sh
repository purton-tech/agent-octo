#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SESSION=agent-octo-com
SCRIPT_NAME="$(basename "${BASH_SOURCE[0]}")"

in_devcontainer() {
  [ -n "${REMOTE_CONTAINERS:-}" ] || [ -n "${CODESPACES:-}" ] || [ -f "/.dockerenv" ]
}

start_tmux() {
  command -v tmux >/dev/null 2>&1 || (sudo apt update && sudo apt install -y tmux)

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

  tmux -f ~/.tmux.conf new-session -d -s "$SESSION" -n site -c "$ROOT"

  tmux select-pane -t "$SESSION:0.0" -T "site"
  tmux send-keys -t "$SESSION:0.0" "just ws" C-m

  tmux split-window -h -t "$SESSION:0" -c "$ROOT"
  tmux select-pane -t "$SESSION:0.1" -T "tailwind"
  tmux send-keys -t "$SESSION:0.1" "just wts" C-m

  tmux select-pane -t "$SESSION:0.0"
  tmux select-layout -t "$SESSION:0" even-horizontal
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

  devcontainer up --workspace-folder /workspace >/dev/null
  exec devcontainer exec --workspace-folder /workspace bash -lc "cd '$ROOT' && ./'$SCRIPT_NAME'"
fi
