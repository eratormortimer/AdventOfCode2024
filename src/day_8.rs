use std::io;
use nalgebra::Vector2;
use std::collections::HashMap;
use std::collections::HashSet;
use itertools::Itertools; // Import itertools
fn wait_for_enter() {
    let mut input = String::new();
    println!("Press Enter to continue...");
    io::stdin().read_line(&mut input).expect("Failed to read line");
}
pub fn calc_antinode_amount(field: Vec<Vec<char>>) -> Option<i64> {
    let mut rtn =0;
    let mut possible_antinodes: Vec<Vector2<i64>> = Vec::new();
    let dimensions = get_dimensions(&field);
    let antennas = get_antennas(&field);
    for (antenna,antenna_locations) in &antennas {
        possible_antinodes.extend(get_antinode_for_antenna(antenna_locations, &dimensions));
    }
    let mut unique_antinode: Vec<Vector2<i64>> = Vec::new();
    for antinode in possible_antinodes {
        if !unique_antinode.contains(&antinode) {
            unique_antinode.push(antinode);
        }
    }
    println!("uniques: {:?}",unique_antinode);
    return Some(unique_antinode.len() as i64);
    for antinode in unique_antinode {
        let mut reject = false; 
        for (antenna,antenna_locations) in &antennas {
            if antenna_locations.contains(&antinode) {
                reject = true;
                println!("excluded: {:?}",antinode);
            }
        }
        if !reject {
            rtn += 1;
        }
    }
    Some(rtn)
}

fn get_antennas(field: &Vec<Vec<char>>) -> HashMap<char, Vec<Vector2<i64>>> {
    let mut rtn : HashMap<char,Vec<Vector2<i64>>> = HashMap::new();
    for (pos_x,line) in field.iter().enumerate() {
        for (pos_y,c) in line.iter().enumerate() {
            if *c != '.' {
                let location = Vector2::new(pos_x as i64, pos_y as i64);
                rtn.entry(*c).or_insert_with(Vec::new).push(location);
            }
        }
    }
    return rtn;
}

fn get_dimensions(field: &Vec<Vec<char>>) -> Vector2<i64> {
    let mut rtn = Vector2::new(field.len() as i64, field[0].len() as i64);
    rtn
}

fn get_antinode_for_antenna(antenna_locations: &Vec<Vector2<i64>>, dimensions: &Vector2<i64>) -> Vec<Vector2<i64>> {
    let mut possible_antinodes: Vec<Vector2<i64>> = Vec::new();
    let combinations = antenna_locations.iter().combinations(2);
    for combination in combinations {
        let (vec_a, vec_b) = (combination[0], combination[1]);
        let diff = vec_a - vec_b;
        let antinode_1 = vec_a + diff;
        let antinode_2 = vec_b - diff;
        if antinode_1.x < dimensions.x && antinode_1.y < dimensions.y && antinode_1.x >=0 && antinode_1.y >= 0 {
            let mut reject = false;
            for vector in &possible_antinodes {
                if vector.x == antinode_1.x && vector.y == antinode_1.y {
                    reject = true;
                }
            }
            if !reject {
                possible_antinodes.push(antinode_1);
                println!("added top: {:?}",antinode_1);
            }else {
                println!("excluded top: {:?}",antinode_1);
            }
        }
        if antinode_2.x < dimensions.x && antinode_2.y < dimensions.y && antinode_2.x >= 0 && antinode_2.y >=0 {
            let mut reject = false;
            for vector in &possible_antinodes {
                if vector.x == antinode_2.x && vector.y == antinode_2.y {
                    reject = true;
                }
            }
            if !reject {
                possible_antinodes.push(antinode_2);
                println!("added top: {:?}",antinode_2);
            } else {
                println!("excluded top: {:?}",antinode_2);
            }
        }
    }
    
    possible_antinodes
}