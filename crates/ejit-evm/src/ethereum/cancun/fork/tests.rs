use std::{io::{BufRead, BufReader, Write}, net::TcpStream, path::PathBuf, sync::Arc};

use crate::ethereum::{cancun::blocks::Block, ethereum_rlp::rlp, ethereum_types::bytes::Bytes};

#[test]
fn test_against_alchemy() {
    let url = std::env::var("ALCHEMY_URL").unwrap();

    let client = reqwest::blocking::Client::new();

    let block = 0x61A80;
    let body = format!(
        r#"{{"id": 1,"jsonrpc": "2.0","method": "debug_getRawBlock","params": ["0x{block:x}"]}}"#
    );

    let resp = client
        .post(url)
        .header("accept", "application/json")
        .header("content-type", "application/json")
        .body(body).send().unwrap();

    assert!(resp.status() == 200);
    let res = resp.text().unwrap();
    let (_, rest) = res.split_once(r#"result":"0x"#).unwrap();
    let (hex, _) = rest.split_once('"').unwrap();

    let bytes : Vec<u8> = hex
        .as_bytes()
        .chunks_exact(2)
        .map(|c| u8::from_str_radix(std::str::from_utf8(c).unwrap(), 16).unwrap())
        .collect();

    let block : Block = rlp::decode_to(&bytes).unwrap();

}