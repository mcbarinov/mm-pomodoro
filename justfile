lint:
    cargo fmt && cargo clippy

lint-fix:
  cargo clippy --fix --lib -p ptimer --allow-dirty