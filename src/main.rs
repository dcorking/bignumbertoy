extern crate bigdecimal;
extern crate num_bigint;
extern crate num_traits;
extern crate rustc_serialize;

use bigdecimal::BigDecimal;
use num_traits::One;
use num_bigint::BigInt;
use rustc_serialize::{Encodable, Encoder};

// silly trait
pub trait Speak {
   fn speak(&self) -> &str;
}

impl Speak for BigDecimal {
    fn speak(&self) -> &str {
        println!("Barking.");
        "Woof!"
    }
}

pub struct SmallThing(BigDecimal);

impl Encodable for SmallThing {
    fn encode<S>(&self, s:SmallThing) -> Result<&str,rustc_serialize::Encoder::Error> {
        "Boo!"
    }
}

fn main() {
    // demonstrate BigInt parsing and multiplication
    let billion: BigInt = "1_000_000_000".parse().unwrap();
    println!("Hello, {} people!", &billion);
    println!("Hello, {} grains of sand!", &billion * &billion);
    println!("Hello, {} atoms!", &billion * &billion * &billion);

    // demonstrate BigDecimal constructors, multiplication and division
    let billionth = BigDecimal::one() / BigDecimal::new(billion.clone(), 0);
    println!("\nbillionth: {}", &billionth);
    println!("billionth of a billionth: {}", &billionth * & billionth);
    println!("billionth of a billionth of a billionth: {} or {}",
             &billionth * &billionth * & billionth,
             &billionth / BigDecimal::new(billion.clone(), 0) / BigDecimal::new(billion.clone(), 0));

    // demonstrate the difference between scaling and division
    let thirty_digits: BigDecimal = "123456789012345678901234567890".parse().unwrap();
    let downscaled_thirty = thirty_digits.clone().with_scale(-18);
    println!("\nIf you downscale a BigDecimal like this: {},\nyou will lose precision and end up with this: {}", &thirty_digits, &downscaled_thirty);
    println!("which are represented internally as\n {:#?}, and\n {:#?}", &thirty_digits, &downscaled_thirty);
    let billion_billion = BigDecimal::new(&billion * &billion, 0);
    let divided_thirty = &thirty_digits / &billion_billion;
    println!("So instead of downscaling, divide it by {}\n to get {}\n, represented internally as\n {:#?}, and\n {:#?} \n",
             &billion_billion, &divided_thirty,
             &billion_billion, &divided_thirty);

    let sound = billion_billion.speak();
    println!("The sound it made: {}", sound);
}
