extern crate bigdecimal;
extern crate num_bigint;

use bigdecimal::BigDecimal;
use num_bigint::BigInt;

fn main() {
    let billion: BigInt = "1_000_000_000".parse().unwrap();
    println!("Hello, {} people!", &billion);
    println!("Hello, {} grains of sand!", &billion * &billion);
    println!("Hello, {} atoms!", &billion * &billion * &billion);

    let one: BigDecimal = "1".parse().unwrap();
    let billionth = one / BigDecimal::new(billion, 0);
    println!("billionth: {}", &billionth);
    println!("billionth of a billionth: {}", &billionth * & billionth);

    let atto_things: BigDecimal = "123456789012345678901234567890".parse().unwrap();
    let atto_things = atto_things.with_scale(-18);
    // I expected 0.123456789012345678901234567890 but
    // I got 123456789012000000000000000000
    println!("LACK OF atto precision: {}", atto_things);
}
