extern crate num_bigint;

use num_bigint::BigInt;

fn main() {
    let billion: BigInt = "1_000_000_000".parse().unwrap();
    println!("Hello, {} people!", billion);
}
