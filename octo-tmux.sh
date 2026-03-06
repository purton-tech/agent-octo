#!/usr/bin/env bash
set -e

SESSION=octo
ROOT=$(pwd)

tmux -f "$ROOT/tmux.conf" new-session -d -s $SESSION -n dev -c "$ROOT"

tmux select-pane -t $SESSION:0.0 -T "shell"

tmux split-window -h -t $SESSION:0 -c "$ROOT"
tmux select-pane -t $SESSION:0.1 -T "octo"
tmux send-keys -t $SESSION:0.1 "just wo" C-m

tmux split-window -v -t $SESSION:0.1 -c "$ROOT"
tmux select-pane -t $SESSION:0.2 -T "db queries"
tmux send-keys -t $SESSION:0.2 "just wd" C-m

tmux split-window -v -t $SESSION:0.2 -c "$ROOT"
tmux select-pane -t $SESSION:0.3 -T "wasm"
tmux send-keys -t $SESSION:0.3 "just wi" C-m

tmux split-window -v -t $SESSION:0.3 -c "$ROOT"
tmux select-pane -t $SESSION:0.4 -T "tailwind"
tmux send-keys -t $SESSION:0.4 "just wtw" C-m

tmux select-pane -t $SESSION:0.0
tmux select-layout -t $SESSION:0 main-vertical

tmux attach -t $SESSION
