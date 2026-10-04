# Fast development loop tasks

# Auto rebuild and restart the desktop app on changes
dev:
    watchexec -r -e rs,toml,json -w crates -w assets -- cargo run -p desktop

# Auto typecheck the workspace on changes
check:
    watchexec -e rs,toml -w crates -- cargo check --workspace
