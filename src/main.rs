mod parse_input;
mod day_1;
mod day_2;
mod day_3;
mod day_4;
fn main() {
    match parse_input::read_input("1") {
        Some(parse_input::Data::Single((first_parts, second_parts))) => {
            // Print the vectors if Some is returned
            match day_1::get_similarity_score(first_parts,second_parts) {
                Some(result) => {
                    println!("The result: {:?}.",result);
                }
                None => {
                    println!("result busted");
                }
            }
        }
        _ => {
            // Handle the None case
            println!("The input vector was empty.");
        }
    }
    match parse_input::read_input("2") {
        Some(parse_input::Data::Nested(list)) => {
            match day_2::calc_safe_amount_with_level(list) {
                Some(safe) => {
                    println!("The result: {:?}.",safe);
                }
                None => {
                    println!("result busted");
                }
            }
            
        }
        _ => {
            println!("result busted");
        }
    }
    match parse_input::read_input("3") {
        Some(parse_input::Data::Muls(list)) => {
            match day_3::calc_muts(list.clone()) {
                Some(result) => {
                    println!("The result: {:?}.",result);
                }
                None => {
                    println!("result busted");
                }
            }
        }
        _ => {
            println!("result busted");
        }
    }
    match parse_input::read_input("4") {
        Some(parse_input::Data::CharArray(list)) => {
            println!("The result: {:?}.",list);
            match day_4::get_xmas(list.clone()) {
                Some(result) => {
                    println!("The result: {:?}.",result);
                }
                None => {
                    println!("result busted");
                }
            }
        }
        _ => {
            println!("result busted");
        }
    }
}


