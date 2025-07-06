mod ean_checksum;
mod rib_checksum;
use std::env;

//use ean_checksum::is_correct_ean;
use rib_checksum::calcul_checksum_rib_from_string;

//use crate::ean_checksum::is_correct_ean;
use crate::ean_checksum::calculate_digit_check;



fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage: {} <mode> <value_test>", args[0]);
        return;
    }

    let value_test = &args[2];
    let mode = &args[1];

    println!("Value tested: {value_test}");
    println!("Mode: {mode}");


    match mode.as_str() {
        "ean" => {
            // Call the EAN checksum function here if needed
            println!("EAN mode selected.");
            
            let r =calculate_digit_check(value_test);
            println!("Ean Digit Check {}",r);
        },
        "rib" => {
            // Call the RIB checksum function
            println!("RIB mode selected.");
            let key = calcul_checksum_rib_from_string(value_test);
            println!("Checksum RIB: {}", key);
        },
        _ => {
            println!("Unknown mode: {}", mode);
            return;
        }
        
    }

    
}
