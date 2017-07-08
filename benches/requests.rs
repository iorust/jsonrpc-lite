#![feature(test)]
extern crate test;
extern crate jsonrpc_lite;

use test::Bencher;
use jsonrpc_lite::JsonRpc;

// As of c7c5dd3d
//   1: 11_649   ns/iter (+/- 951  )
//   2: 11_623   ns/iter (+/- 834  )
//   3: 11_811   ns/iter (+/- 492  )
//   4: 11_739   ns/iter (+/- 284  )
//   5: 11_774   ns/iter (+/- 275  )
// ---------------------------------
// avg: 11_719.2 ns/iter (+/- 567.2)
const COMPLEX: &str = include_str!("request_complex.json");
#[bench]
fn parse_complex(b: &mut Bencher) {
    b.iter(|| {
        JsonRpc::from_str(COMPLEX).expect("Unable to parse input");
    });
}

// As of c7c5dd3d
//   1: 4_435   ns/iter (+/- 284)
//   2: 4_545   ns/iter (+/- 249)
//   3: 4_567   ns/iter (+/- 318)
//   4: 4_547   ns/iter (+/- 227)
//   5: 4_528   ns/iter (+/- 112)
// ------------------------------
// avg: 4_524.4 ns/iter (+/- 238)
const SIMPLE: &str = include_str!("request_simple.json");
#[bench]
fn parse_simple(b: &mut Bencher) {
    b.iter(|| {
        JsonRpc::from_str(SIMPLE).expect("Unable to parse input");
    });
}
