mod ean_checksum;
use std::env;

use ean_checksum::is_correct_ean;


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage: {} <value_test> <mode>", args[0]);
        return;
    }

    let value_test = &args[1];
    let mode = &args[2];
    println!("Value tested: {value_test}");
    println!("Mode: {mode}");

    let r =is_correct_ean(value_test);
    println!("{}",r);
}
