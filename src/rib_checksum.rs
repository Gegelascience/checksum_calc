pub fn calcul_checksum_rib(code_banque:u32,code_guichet:u32,numero_compte:&str) -> u32 {
    if numero_compte.len() != 11 {
        panic!("numero_compte must be 11 characters long");
    }

    // Calculate the checksum
    let mut num_compte_number:u64 = 0;
    for (_i, c) in numero_compte.chars().enumerate() {
        let digit:u32;
        if !c.is_numeric() {
            digit = corresponding_number_char(c.to_ascii_uppercase());
            if digit == 0 {
                panic!("Invalid character in numero_compte: {}", c);
            }
        } else {
            digit = c.to_digit(10).unwrap();
            
        }
        println!("Character: {}, Corresponding digit: {}", c, digit);
        num_compte_number = num_compte_number * 10 + digit as u64;
        
    }

    // The checksum is the last two digits of the sum
    return (97 - ((code_banque as u64 * 89 + code_guichet as u64 * 15 + num_compte_number*3) % 97)) as u32;
    
}

fn corresponding_number_char(c: char) -> u32 {

    match c {
        'A' => 1,
        'B' => 2,
        'C' => 3,
        'D' => 4,
        'E' => 5,
        'F' => 6,
        'G' => 7,
        'H' => 8,
        'I' => 9,
        'J' => 1,
        'K' => 2,
        'L' => 3,
        'M' => 4,
        'N' => 5,
        'O' => 6,
        'P' => 7,
        'Q' => 8,
        'R' => 9,
        'S' => 2,
        'T' => 3,
        'U' => 4,
        'V' => 5,
        'W' => 6,
        'X' => 7,
        'Y' => 8,
        'Z' => 9,
        _ => panic!("Invalid character in numero_compte: {}", c),
    }
}



pub fn calcul_checksum_rib_from_string(rib:&str) -> u32 {
    if rib.len() != 23 {
        panic!("RIB must be 23 characters long");
    }
    
    let code_banque = rib[0..5].parse::<u32>().unwrap();
    let code_guichet = rib[5..10].parse::<u32>().unwrap();
    let numero_compte = &rib[10..21];
    
    return calcul_checksum_rib(code_banque,code_guichet,numero_compte);
}