#![feature(test)]
extern crate test;
extern crate jsonrpc_lite;

use test::Bencher;
use jsonrpc_lite::JsonRpc;

// ~6_000 ns/iter
const COMPLEX: &str = include_str!("request_complex.json");
#[bench]
fn parse_complex(b: &mut Bencher) {
    b.iter(|| {
        JsonRpc::parse(COMPLEX).expect("Unable to parse input");
    });
}

// ~2_000 ns/iter
const SIMPLE: &str = include_str!("request_simple.json");
#[bench]
fn parse_simple(b: &mut Bencher) {
    b.iter(|| {
        JsonRpc::parse(SIMPLE).expect("Unable to parse input");
    });
}
