# Performance Benchmarking

## Requirements

- Rustc 1.74+

## Setup

### Basic Dependencies

Install [Rust](https://www.rust-lang.org/tools/install) using Rustup:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Prepare for NV Gitlab private crates

To pull crates from Gitlab:

```bash
mkdir .cargo
touch .cargo/config.toml
cat << EOF > .cargo/config.toml
[net]
git-fetch-with-cli = true
EOF
```

```bash
cargo watch -c -w src -x check
```

Do the load test:

```bash
cargo run -- --host "http://127.0.0.1:5001" --report-file=report.html -u 16
```

