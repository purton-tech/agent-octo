
codex: 
    sudo npm install -g @openai/codex

bot:
    mold -run cargo watch --workdir /workspace --no-gitignore -x "run -p octo --bin octo"
