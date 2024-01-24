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

### Example

Request

```
toji -r "https://arbitrum-sepolia.publicnode.com" -n 331612
```

Response

```
Raw Block Header  :EvmBlockHeader { parent_hash: "0x40a0b2f9b4eb33242268de3003e64779a3b1292a5cd56529af9dfff8523e7a60", uncle_hash: "0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347", coinbase: "0xa4b000000000000000000073657175656e636572", state_root: "0x21894e4bbacc76e11bf0e9fbc8170e75d8f03047f4d4dfa3b13634833d14ee80", transactions_root: "0x08f4b87f4465e58296af2672b30596181bf3a7c770311e62cdc357896be7b393", receipts_root: "0x4ab51dcac2b8ef2e196d9e3363683b7c8720c0a6664cf2cf7ed89adfd2b71e0a", logs_bloom: "0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000", difficulty: 1, number: 331612, gas_limit: 1125899906842624, gas_used: 37120, timestamp: 1694514298, extra_data: "0xb5d8a5da3828de54e5a1d8a9624a799331ddac5100bd2df5d6cc11cdb990521a", mix_hash: "0x00000000000016830000000000413418000000000000000a0000000000000000", nonce: "0x0000000000024167", base_fee_per_gas: Some(100000000), withdrawals_root: None }

RLP Encoded Block Header :"f90222a040a0b2f9b4eb33242268de3003e64779a3b1292a5cd56529af9dfff8523e7a60a01dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d4934794a4b000000000000000000073657175656e636572a021894e4bbacc76e11bf0e9fbc8170e75d8f03047f4d4dfa3b13634833d14ee80a008f4b87f4465e58296af2672b30596181bf3a7c770311e62cdc357896be7b393a04ab51dcac2b8ef2e196d9e3363683b7c8720c0a6664cf2cf7ed89adfd2b71e0ab90100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000183050f5c87040000000000008291008465003c7aa0b5d8a5da3828de54e5a1d8a9624a799331ddac5100bd2df5d6cc11cdb990521aa000000000000016830000000000413418000000000000000a00000000000000008800000000000241678405f5e100"

Block Hash :"0xe278dc4590304d2f0579689fb1bdd76a08ceac0e0e37bc426e1357e5d8395e1c"
```
