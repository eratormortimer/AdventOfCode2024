use std::io;

fn wait_for_enter() {
    let mut input = String::new();
    println!("Press Enter to continue...");
    io::stdin().read_line(&mut input).expect("Failed to read line");
}

pub fn how_many_stones(mut input: &mut Vec<i64>) -> Option<usize> {
    for i in 0..75 {
        transform(&mut input);
        //println!("In {:?} iteration: {:?}",i,&input);
    }
    Some(input.len())
} 

fn transform(input: &mut Vec<i64>) {
    let input_clone = input.clone();
    let mut modifier: usize = 0;
    for (index,element) in input_clone.iter().enumerate() {
        if *element == 0 {
            input[index+modifier] = 1;
        } else if element.to_string().len()%2 == 0 {
            let temp = element.to_string();
            let (left,right) = divide_string_in_half(&temp);
            input[index+modifier] = left.parse::<i64>().unwrap();;
            input.insert(index+modifier+1,right.parse::<i64>().unwrap());
            modifier += 1;
            //println!("mod: {:?}, left: {:?}, right: {:?}",modifier,left,right);
        } else {
            input[index+modifier] = element*2024;
        }
    }
}

fn divide_string_in_half(s: &str) -> (&str, &str) {
    let mid = s.len() / 2;
    s.split_at(mid)
}