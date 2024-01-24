use alloy_primitives::{keccak256, Bytes};
use alloy_rlp::{BytesMut, Encodable, RlpDecodable, RlpEncodable};
use clap::Parser;
use reqwest::header;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;

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

#[derive(Debug, RlpDecodable, RlpEncodable, PartialEq)]
#[rlp(trailing)]
pub struct EvmBlockHeader {
    pub parent_hash: String,
    pub uncle_hash: String,
    pub coinbase: String,
    pub state_root: String,
    pub transactions_root: String,
    pub receipts_root: String,
    pub logs_bloom: String,
    pub difficulty: u64,
    pub number: u64,
    pub gas_limit: u64,
    pub gas_used: u64,
    pub timestamp: u64,
    pub extra_data: String,
    pub mix_hash: String,
    pub nonce: String,
    pub base_fee_per_gas: Option<u64>,
    pub withdrawals_root: Option<String>,
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
}

impl From<&EvmBlockHeaderFromRpc> for EvmBlockHeader {
    fn from(value: &EvmBlockHeaderFromRpc) -> Self {
        Self {
            parent_hash: value.parent_hash.clone(),
            uncle_hash: value.sha3_uncles.clone(),
            coinbase: value.miner.clone(),
            state_root: value.state_root.clone(),
            transactions_root: value.transactions_root.clone(),
            receipts_root: value.receipts_root.clone(),
            logs_bloom: value.logs_bloom.clone(),
            difficulty: u64::from_str_radix(&value.difficulty.as_str()[2..], 16).unwrap(),
            number: u64::from_str_radix(&value.number.as_str()[2..], 16).unwrap(),
            gas_limit: u64::from_str_radix(&value.gas_limit.as_str()[2..], 16).unwrap(),
            gas_used: u64::from_str_radix(&value.gas_used.as_str()[2..], 16).unwrap(),
            timestamp: u64::from_str_radix(&value.timestamp.as_str()[2..], 16).unwrap(),
            extra_data: value.extra_data.clone(),
            mix_hash: value.mix_hash.clone(),
            nonce: value.nonce.clone(),
            base_fee_per_gas: value
                .clone()
                .base_fee_per_gas
                .map(|x| u64::from_str_radix(&x.as_str()[2..], 16).unwrap()),
            withdrawals_root: value.withdrawals_root.clone(),
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
                    serde_json::Value::Bool(true),
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
    let header: EvmBlockHeader = EvmBlockHeader::from(&header_rpc);

    let mut extradata = BytesMut::new();
    header.encode(&mut extradata);
    let rlp_encoded_bytes: Bytes = extradata.freeze().into();
    //let rlp_decoded_header = EvmBlockHeader::decode(&mut buffer.as_slice()).unwrap();
    // let hex_encoded_header = hex::encode(&rlp_encoded_bytes);
    let hex_encoded_header = hex::encode(&rlp_encoded_bytes);
    let block_hash = keccak256(&rlp_encoded_bytes);

    println!("Hex encoded block header :{:?}", rlp_encoded_bytes);
    println!("Block Hash :{:?}", block_hash);
    println!("Block Hash :{:?}", hex::encode(block_hash));
    // println!("Block Hash :{:?}", block_hash);
}
