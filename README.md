# Galaxy-training

This is a tool that automatically creates groups and assign the training role based on a schedule. This role can be used when routing with TPV to reverse compute resources.

## Installation

```bash
cargo install https://github.com/laperlej/galaxy-training
```
## Usage

```bash
training-manager <config-file>
```
the config file is a toml with this format:

```toml
[groups]
team_a = ["alice@example.com", "bob@example.com"]
team_b = ["charlie@example.com"]

[schedule]
team_a = [
    { from = "2023-01-01", to = "2023-06-30" },
    { from = "2023-07-01", to = "2023-12-31" }
]
team_b = [
    { from = "2023-01-01", to = "2023-12-31" }
]
```
