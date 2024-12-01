use std::fs;

pub fn read_input() -> Option<(Vec<String>, Vec<String>)> {

    // Specify the file path
    let file_path = "input_1.txt";

    match fs::read_to_string(file_path) {
        Ok(content) => read_into_lists(&content),
        Err(e) => {
            eprintln!("Error reading file {}: {}", file_path, e);
            None // Return an empty string on error
        }
    }
}

fn read_into_lists(input: &str)->Option<(Vec<String>, Vec<String>)> {
    let lines = input.split("\n").collect::<Vec<_>>();
    let mut lines_left: Vec<String> = Vec::new();
    let mut lines_right: Vec<String> = Vec::new();
    for line in &lines {
        let parts: Vec<&str> = line.split_whitespace().collect();
        lines_left.push(parts[0].to_string());
        lines_right.push(parts[1].to_string());
    }
    Some((lines_left,lines_right))
}