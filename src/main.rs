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
    println!("billionth of a billionth of a billionth: {}", &billionth * &billionth * & billionth);

    let thirty_digits: BigDecimal = "123456789012345678901234567890".parse().unwrap();
    let downscaled_thirty = thirty_digits.clone().with_scale(-18);
    println!("If you downscale a BigDecimal like this: {}, you will lose precision and end up with this: {}", &thirty_digits, &downscaled_thirty);
}
