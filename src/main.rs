use std::collections::HashMap;
mod parse_input;

fn main() {
    match parse_input::read_input() {
        Some((first_parts, second_parts)) => {
            // Print the vectors if Some is returned
            match get_similarity_score(first_parts,second_parts) {
                Some(result) => {
                    println!("The result: {:?}.",result);
                }
                None => {
                    println!("result busted");
                }
            }
        }
        None => {
            // Handle the None case
            println!("The input vector was empty.");
        }
    }
}

fn get_distance(list_1:Vec<String>, list_2:Vec<String>) -> Option<i32> {
     // Convert strings to integers
    let mut list_1: Vec<i32> = list_1
    .into_iter()
    .map(|s| s.parse::<i32>().unwrap_or(0)) // Parse each string to i32, defaulting to 0 if it fails
    .collect();
    let mut list_2: Vec<i32> = list_2
    .into_iter()
    .map(|s| s.parse::<i32>().unwrap_or(0)) // Parse each string to i32, defaulting to 0 if it fails
    .collect();
 // Sort the numbers
    list_1.sort();
    list_2.sort();
    let mut distances: Vec<i32> = Vec::new();
    for (element_1, element_2) in list_1.iter().zip(list_2.iter()) {
        distances.push((element_1 - element_2).abs());
    }
    let mut rtn = 0;
    for element in distances.iter() {
        rtn = rtn + element;
    }
    Some(rtn)
}

fn get_similarity_score(list_1:Vec<String>, list_2:Vec<String>) -> Option<i32> {
    // Convert strings to integers
    let mut list_1: Vec<i32> = list_1
    .into_iter()
    .map(|s| s.parse::<i32>().unwrap_or(0)) // Parse each string to i32, defaulting to 0 if it fails
    .collect();
    let mut list_2: Vec<i32> = list_2
    .into_iter()
    .map(|s| s.parse::<i32>().unwrap_or(0)) // Parse each string to i32, defaulting to 0 if it fails
    .collect();
    let mut occurrences = HashMap::new();

    for element in list_2 {
        *occurrences.entry(element).or_insert(0) += 1;
    }
    let mut similarities: Vec<i32> = Vec::new();
    for element_1 in list_1 {
        similarities.push(element_1 * occurrences.get(&element_1).unwrap_or(&0));
    }
    let rtn: i32 = similarities.iter().sum();
    Some(rtn)
}
