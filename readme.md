## Toji

Blockchain toolkit: CLI application to get `rlp encoded block header`, `raw block header`, `block hash` mainly purpose for debugging block hash/header calculation.

### Installation

```
cargo install --git https://github.com/rkdud007/toji --locked --force
```

### Usage

```
Usage: toji --rpc-url <RPC_URL> --block-number <BLOCK_NUMBER>

Options:
  -r, --rpc-url <RPC_URL>            The RPC endpoint
  -n, --block-number <BLOCK_NUMBER>  The block number to query
  -h, --help                         Print help
```
