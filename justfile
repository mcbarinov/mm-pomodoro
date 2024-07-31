lint:
    cargo fmt && cargo clippy

lint-fix:
  cargo clippy --fix --lib -p mm-pomodoro --allow-dirty

publish:
    cargo publish
