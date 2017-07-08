#![feature(test)]
extern crate test;
extern crate jsonrpc_lite;

use test::Bencher;
use jsonrpc_lite::JsonRpc;

// As of c7c5dd3d
//   1: 116_213   ns/iter (+/-  6_371  )
//   2: 117_091   ns/iter (+/-  3_634  )
//   3: 138_441   ns/iter (+/-  2_482  )
//   4: 116_589   ns/iter (+/- 23_721  )
//   5: 115_867   ns/iter (+/-  8_039  )
// -------------------------------------
// avg: 120_840.2 ns/iter (+/-  8_849.4)
const BATCH_SHORT: &str = include_str!("short_batch.json");
#[bench]
fn parse_short(b: &mut Bencher) {
    b.iter(|| { JsonRpc::from_str_vec(BATCH_SHORT); });
}

// As of c7c5dd3d
//   1: 148_369   ns/iter (+/- 6_788  )
//   2: 150_570   ns/iter (+/- 3_757  )
//   3: 162_508   ns/iter (+/- 2_661  )
//   4: 147_791   ns/iter (+/- 4_352  )
//   5: 146_226   ns/iter (+/- 9_569  )
// ------------------------------------
// avg: 151_092.8 ns/iter (+/- 5_425.4)
const BATCH_REQ_RES: &str = include_str!("batch_request_response.json");
#[bench]
fn parse_request_response(b: &mut Bencher) {
    b.iter(|| { JsonRpc::from_str_vec(BATCH_REQ_RES); });
}
