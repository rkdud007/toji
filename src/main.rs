use accumulators::hasher::{keccak::KeccakHasher, Hasher};
use rlp::{Encodable, RlpStream};
// use alloy_rlp::{BytesMut, Encodable, RlpDecodable, RlpEncodable};
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

#[derive(Debug, PartialEq)]
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

impl Encodable for EvmBlockHeader {
    fn rlp_append(&self, s: &mut RlpStream) {
        let with_base_fee_per_gas = if self.base_fee_per_gas.is_some() {
            1
        } else {
            0
        };

        let with_withdrawals_root = if self.withdrawals_root.is_some() {
            1
        } else {
            0
        };

        let len = 15 + with_base_fee_per_gas + with_withdrawals_root;

        s.begin_list(len);
        s.append(&hex::decode(&self.parent_hash[2..]).unwrap());
        s.append(&hex::decode(&self.uncle_hash[2..]).unwrap());
        s.append(&hex::decode(&self.coinbase[2..]).unwrap());
        s.append(&hex::decode(&self.state_root[2..]).unwrap());
        s.append(&hex::decode(&self.transactions_root[2..]).unwrap());
        s.append(&hex::decode(&self.receipts_root[2..]).unwrap());
        s.append(&hex::decode(&self.logs_bloom[2..]).unwrap());
        if self.difficulty == 0 {
            s.append(&hex::decode(&"0x"[2..]).unwrap());
        } else {
            s.append(&self.difficulty);
        }
        s.append(&self.number);
        s.append(&self.gas_limit);
        s.append(&self.gas_used);
        s.append(&self.timestamp);
        s.append(&hex::decode(&self.extra_data[2..]).unwrap());
        s.append(&hex::decode(&self.mix_hash[2..]).unwrap());
        s.append(&hex::decode(&self.nonce[2..]).unwrap());
        if let Some(x) = self.base_fee_per_gas {
            s.append(&x);
        }
        if let Some(x) = &self.withdrawals_root {
            s.append(&hex::decode(&x[2..]).unwrap());
        }
    }
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
    let rlp = hex::encode(rlp::encode(&header));

    // let mut extradata = BytesMut::new();
    // header.encode(&mut extradata);
    // let rlp_encoded_bytes: Bytes = extradata.freeze().into();
    //let rlp_decoded_header = EvmBlockHeader::decode(&mut buffer.as_slice()).unwrap();
    // let hex_encoded_header = hex::encode(&rlp_encoded_bytes);
    // let hex_encoded_header = hex::encode(&rlp_encoded_bytes);
    let hasher = KeccakHasher::new();
    let block_hash = hasher.hash(vec![rlp]).unwrap();

    // println!("Hex encoded block header :{:?}", rlp);
    // println!("Block Hash :{:?}", block_hash);
    // println!("Block Hash :{:?}", hex::encode(block_hash));
    println!("Block Hash :{:?}", block_hash);
}