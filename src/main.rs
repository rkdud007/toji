use alloy_primitives::keccak256;
use clap::Parser;
use reqwest::header;
use reth_primitives::{Bloom, Bytes, Header, H160, H256};
use reth_rlp::Encodable;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{str::FromStr, sync::Arc};

#[derive(Debug, Parser)]
struct Cli {
    #[arg(short = 'r', long)]
    #[arg(value_name = "RPC_URL")]
    #[arg(help = "The RPC endpoint")]
    rpc_url: String,

    #[arg(short = 'n', long)]
    #[arg(value_name = "BLOCK_NUMBER")]
    #[arg(help = "The block number to query")]
    block_number: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EvmBlockHeaderFromRpc {
    pub number: String,
    pub hash: String,
    pub difficulty: String,
    pub extra_data: String,
    pub gas_limit: String,
    pub gas_used: String,
    pub logs_bloom: String,
    pub miner: String,
    pub mix_hash: String,
    pub nonce: String,
    pub parent_hash: String,
    pub receipts_root: String,
    pub sha3_uncles: String,
    pub size: String,
    pub state_root: String,
    pub timestamp: String,
    pub total_difficulty: String,
    pub transactions_root: String,
    pub base_fee_per_gas: Option<String>,
    pub withdrawals_root: Option<String>,
    pub blob_gas_used: Option<String>,
    pub excess_blob_gas: Option<String>,
    pub parent_beacon_block_root: Option<String>,
}

impl From<&EvmBlockHeaderFromRpc> for Header {
    fn from(value: &EvmBlockHeaderFromRpc) -> Self {
        Self {
            parent_hash: H256::from_str(&value.parent_hash).unwrap(),
            ommers_hash: H256::from_str(&value.sha3_uncles).unwrap(),
            beneficiary: H160::from_str(&value.miner).unwrap(),
            state_root: H256::from_str(&value.state_root).unwrap(),
            transactions_root: H256::from_str(&value.transactions_root).unwrap(),
            receipts_root: H256::from_str(&value.receipts_root).unwrap(),
            logs_bloom: Bloom::from_str(&value.logs_bloom).unwrap(),
            difficulty: value.difficulty.parse().unwrap(),
            number: u64::from_str_radix(value.number.trim_start_matches("0x"), 16).unwrap(),
            gas_limit: u64::from_str_radix(value.gas_limit.trim_start_matches("0x"), 16).unwrap(),
            gas_used: u64::from_str_radix(value.gas_used.trim_start_matches("0x"), 16).unwrap(),
            timestamp: u64::from_str_radix(value.timestamp.trim_start_matches("0x"), 16).unwrap(),
            extra_data: Bytes::from_str(&value.extra_data).unwrap(),
            mix_hash: H256::from_str(&value.mix_hash).unwrap(),
            nonce: u64::from_str_radix(value.nonce.trim_start_matches("0x"), 16).unwrap(),
            base_fee_per_gas: value
                .base_fee_per_gas
                .as_ref()
                .map(|x| u64::from_str_radix(x.trim_start_matches("0x"), 16).unwrap()),
            withdrawals_root: value
                .withdrawals_root
                .as_ref()
                .map(|x| H256::from_str(x).unwrap()),
            blob_gas_used: value
                .blob_gas_used
                .as_ref()
                .map(|x| u64::from_str_radix(x.trim_start_matches("0x"), 16).unwrap()),
            excess_blob_gas: value
                .excess_blob_gas
                .as_ref()
                .map(|x| u64::from_str_radix(x.trim_start_matches("0x"), 16).unwrap()),
            parent_beacon_block_root: value
                .parent_beacon_block_root
                .as_ref()
                .map(|x| H256::from_str(x).unwrap()),
        }
    }
}

#[derive(Serialize)]
struct GetBlockByNumberRequestBody {
    pub method: String,
    pub params: Vec<serde_json::Value>,
    pub id: u32,
    pub jsonrpc: String,
}

#[derive(Clone, Debug)]
pub struct JsonRpcRequester {
    client: Arc<reqwest::Client>,
    url: String,
}

impl JsonRpcRequester {
    fn new(url: String) -> Self {
        Self {
            client: Arc::new(reqwest::Client::new()),
            url,
        }
    }

    async fn send_request(&self, block_number: usize) -> Result<reqwest::Response, reqwest::Error> {
        self.client
            .post(self.url.clone())
            .header(header::CONTENT_TYPE, "application/json")
            .json(&GetBlockByNumberRequestBody {
                method: "eth_getBlockByNumber".to_string(),
                params: vec![
                    serde_json::Value::String(format!("0x{:x}", block_number)),
                    serde_json::Value::Bool(false),
                ],
                id: 1,
                jsonrpc: "2.0".to_string(),
            })
            .send()
            .await
    }
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();
    let requester = JsonRpcRequester::new(args.rpc_url);
    let response = requester.send_request(args.block_number).await.unwrap();
    if response.status() != 200 {
        return println!("Request failed with status code {}", response.status());
    }
    let body: Value = response.json().await.unwrap();

    let header_rpc: EvmBlockHeaderFromRpc = serde_json::from_value(body["result"].clone()).unwrap();
    let header: Header = Header::from(&header_rpc);

    let mut buffer = Vec::<u8>::new();
    header.encode(&mut buffer);

    let rlp = alloy_primitives::hex::encode(buffer);
    let rlp_decode = hex::decode(rlp.clone()).unwrap();

    let block_hash = keccak256(rlp_decode);
    println!("Raw Block Header  :{:?}\n", header);
    println!("RLP Encoded Block Header :{:?}\n", rlp);
    println!("Block Hash :{:?}\n", block_hash);
}
