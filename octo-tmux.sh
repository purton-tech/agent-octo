#!/usr/bin/env bash
set -euo pipefail

sudo apt update
sudo apt install -y tmux curl

curl -L https://github.com/gitui-org/gitui/releases/download/v0.28.0/gitui-linux-x86_64.tar.gz \
  | sudo tar -xz -C /usr/local/bin --wildcards --strip-components=1 '*/gitui'

cat > ~/.tmux.conf <<'EOF'
set -g status on
set -g status-position bottom

set -g pane-border-status top
set -g pane-border-format "#{pane_title}"

set-option -g mouse on
EOF

SESSION=octo
ROOT="$(pwd)"

if tmux has-session -t "$SESSION" 2>/dev/null; then
  exec tmux attach -t "$SESSION"
fi

# DEV WINDOW (FIRST)
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

# GITUI WINDOW (SECOND)
tmux new-window -t "$SESSION" -n gitui -c "$ROOT"
tmux send-keys -t "$SESSION:1" "gitui" C-m

# START ON DEV
tmux select-window -t "$SESSION:0"

exec tmux attach -t "$SESSION"