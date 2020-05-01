# pingbot

A bot which ping ping ping frequently.

Remind you to reply the issues.

## Quick Start

Copy `config.example.toml` to `config.toml`, update your GitHub token and watched repos.

Token can be generated from here [https://github.com/settings/tokens/new](https://github.com/settings/tokens/new).

```sh
cargo build --release
./target/release/pingbot -c config.toml
```
