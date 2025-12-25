# Setup

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

# Run tests

```bash
cargo test -- --nocapture --test-threads=1
cargo test -- --nocapture --test-threads=1 --include-ignored
```
