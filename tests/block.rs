use alloy_primitives::{keccak256, FixedBytes, U256};
use reth_primitives::{Bytes, Header, H256};
use reth_rlp::{Decodable, Encodable};
use std::str::FromStr;

#[test]
fn test_decode_block_header() {
    let pure = "f901f9a00000000000000000000000000000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000000000000000000000940000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000000000000000000000b90100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000008208ae820d0582115c8215b3821a0a827788a00000000000000000000000000000000000000000000000000000000000000000880000000000000000".to_string();
    let data = hex::decode("f901f9a00000000000000000000000000000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000000000000000000000940000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000000000000000000000b90100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000008208ae820d0582115c8215b3821a0a827788a00000000000000000000000000000000000000000000000000000000000000000880000000000000000").unwrap();
    let expected = Header {
        difficulty: U256::from(0x8aeu64),
        number: 0xd05u64,
        gas_limit: 0x115cu64,
        gas_used: 0x15b3u64,
        timestamp: 0x1a0au64,
        extra_data: Bytes::from_str("7788").unwrap(),
        ommers_hash: H256::zero(),
        state_root: H256::zero(),
        transactions_root: H256::zero(),
        receipts_root: H256::zero(),
        ..Default::default()
    };

    let mut buffer = Vec::<u8>::new();
    expected.encode(&mut buffer);
    let header = <Header as Decodable>::decode(&mut data.as_slice()).unwrap();
    assert_eq!(header, expected);
    assert_eq!(buffer, data);
    assert_eq!(hex::encode(buffer.clone()), hex::encode(data));
    assert_eq!(hex::encode(buffer.clone()), pure);

    let is_this_blockhash = keccak256(buffer.clone());

    let expected_hash_with_alloy =
        FixedBytes::from_str("8c2f2af15b7b563b6ab1e09bed0e9caade7ed730aec98b70a993597a797579a9")
            .unwrap();

    let expected_hash_with_reth =
        H256::from_str("8c2f2af15b7b563b6ab1e09bed0e9caade7ed730aec98b70a993597a797579a9").unwrap();
    assert_eq!(header.hash_slow(), expected_hash_with_reth);
    assert_eq!(is_this_blockhash, expected_hash_with_alloy);
}
