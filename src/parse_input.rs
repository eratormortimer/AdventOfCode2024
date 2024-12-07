use std::fs;
use regex::Regex;

#[derive(Debug)]
pub enum Data {
    Single((Vec<String>, Vec<String>)),
    Nested(Vec<Vec<String>>),
    Muls(Vec<(i32,i32)>),
    CharArray(Vec<Vec<char>>)
}

pub fn read_input(day: &str) -> Option<Data> {

    // Specify the file path
    let file_path = format!("input_{}.txt", day);
    
    match fs::read_to_string(file_path.clone()) {
        Ok(content) => {
            if day == "1" { 
                return Some(Data::Single(read_into_lists(&content)));
            }
            if day == "2" {
                return Some(Data::Nested(read_into_list_of_lists(&content)));
            }
            if day == "3" {
                return Some(Data::Muls(get_do_muls(&content)));
            }
            if day == "4" {
                return Some(Data::CharArray(read_into_char_array(&content)));
            }
            return None;
        }
        Err(e) => {
            eprintln!("Error reading file {}: {}", file_path, e);
            None // Return an empty string on error
        }
    }
    
    

}

fn read_into_lists(input: &str) -> (Vec<String>,Vec<String>) {
    let lines = input.split("\n").collect::<Vec<_>>();
    let mut lines_left: Vec<String> = Vec::new();
    let mut lines_right: Vec<String> = Vec::new();
    for line in &lines {
        let parts: Vec<&str> = line.split_whitespace().collect();
        lines_left.push(parts[0].to_string());
        lines_right.push(parts[1].to_string());
    }
    (lines_left,lines_right)
}

fn read_into_list_of_lists(input: &str) -> Vec<Vec<String>> {
    let lines = input.lines();
    let rtn = lines.map(|line| line.split_whitespace().map(|word| word.to_string()).collect())
    .collect();
    rtn
}

fn read_into_char_array(input: &str) -> Vec<Vec<char>> {
    let lines = input.lines();
    let mut rtn: Vec<Vec<char>> = Vec::new();
    for line in lines {
        rtn.push(line.chars().collect());
    }
    rtn
}

fn read_into_muls(input: Vec<String>) -> Vec<(i32,i32)> {
    let lines = input;
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut rtn: Vec<(i32,i32)> = Vec::new();
    for line in lines {
        for matching in re.captures_iter(&line) {
            let first: i32 = matching.get(1).map_or(0, |m| m.as_str().parse().unwrap_or(0));
            let second: i32 = matching.get(2).map_or(0, |m| m.as_str().parse().unwrap_or(0));
            rtn.push((first,second));
        }
    }
    rtn
}

fn get_do_muls(input: &str) -> Vec<(i32,i32)> {
    let mut lines: Vec<String> = Vec::new();
    let re_do = Regex::new(r"do\(\)").unwrap();
    let re_dont = Regex::new(r"don't\(\)").unwrap();
    let mut split: Vec<&str> = input.split("don't()").collect();
    lines.push(split[0].to_string());
    split.remove(0);
    for element in split {
        println!("{:?}",element);
        println!("NEW LINE");
        let element_split: Vec<String> = element.split("do()").map(|s| s.to_string()).collect();
        lines.extend(element_split.iter().skip(1).cloned());
    }
    read_into_muls(lines)
}       
