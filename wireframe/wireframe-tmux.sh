#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORKSPACE_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
SESSION="wireframe"
TMUX_CONFIG="/tmp/wireframe-tmux.conf"

in_devcontainer() {
  [ -n "${REMOTE_CONTAINERS:-}" ] || [ -n "${CODESPACES:-}" ] || [ -f "/.dockerenv" ]
}

ensure_tmux() {
  command -v tmux >/dev/null 2>&1 || {
    sudo apt update
    sudo apt install -y tmux
  }
}

write_tmux_config() {
  cat > "$TMUX_CONFIG" <<'EOF'
set -g status on
set -g status-position bottom
set -g pane-border-status top
set -g pane-border-format "#{pane_title}"
set-option -g mouse on
bind -n F1 select-window -t 0
EOF
}

attach_session() {
  if [[ -n "${TMUX:-}" ]] && tmux display-message -p '#S' >/dev/null 2>&1; then
    exec tmux switch-client -t "$SESSION"
  fi

  exec tmux attach-session -t "$SESSION"
}

start_tmux() {
  ensure_tmux
  write_tmux_config
  mkdir -p "${SCRIPT_DIR}/dist"

  if tmux has-session -t "$SESSION" 2>/dev/null; then
    attach_session
  fi

  tmux -f "$TMUX_CONFIG" new-session -d -s "$SESSION" -n wireframe -c "$SCRIPT_DIR"

  tmux select-pane -t "$SESSION:0.0" -T "server"
  tmux send-keys -t "$SESSION:0.0" "python3 -m http.server 8000" C-m

  tmux split-window -h -t "$SESSION:0" -c "$SCRIPT_DIR"
  tmux select-pane -t "$SESSION:0.1" -T "tailwind"
  tmux send-keys -t "$SESSION:0.1" "tailwind-extra -i ./input.css -o ./dist/tailwind.css --watch" C-m

  tmux select-layout -t "$SESSION:0" even-horizontal
  attach_session
}

if in_devcontainer; then
  start_tmux
else
  command -v devcontainer >/dev/null 2>&1 || {
    echo "devcontainer CLI is not installed."
    echo "Install it with: npm install -g @devcontainers/cli"
    exit 1
  }

  devcontainer up --workspace-folder "$WORKSPACE_ROOT" >/dev/null
  exec devcontainer exec --workspace-folder "$WORKSPACE_ROOT" bash -lc "cd '$SCRIPT_DIR' && ./wireframe-tmux.sh"
fi
