#![feature(test)]
extern crate test;
extern crate jsonrpc_lite;

use test::Bencher;
use jsonrpc_lite::JsonRpc;

// ~76_000 ns/iter
const BATCH_SHORT: &str = include_str!("short_batch.json");
#[bench]
fn parse_short(b: &mut Bencher) {
    b.iter(|| {
        JsonRpc::parse_vec(BATCH_SHORT).expect("Unable to parse input");
    });
}

// ~94_000 ns/iter
const BATCH_REQ_RES: &str = include_str!("batch_request_response.json");
#[bench]
fn parse_request_response(b: &mut Bencher) {
    b.iter(|| {
        JsonRpc::parse_vec(BATCH_REQ_RES).expect("Unable to parse input");
    });
}
