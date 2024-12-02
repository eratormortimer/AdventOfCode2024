use std::fs;

#[derive(Debug)]
pub enum Data {
    Single((Vec<String>, Vec<String>)),
    Nested(Vec<Vec<String>>),
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
