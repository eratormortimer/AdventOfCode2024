use std::io;

fn wait_for_enter() {
    let mut input = String::new();
    println!("Press Enter to continue...");
    io::stdin().read_line(&mut input).expect("Failed to read line");
}

pub fn get_middle_numbers(order: Vec<(i32,i32)>, lists: Vec<Vec<i32>>) -> Option<i32> {
    let mut rtn: i32 = 0;
    let (mut lists, mut incorrect_lists) =  get_correct_order(&order,&lists);
    fix_incorrect_lists(&order,&mut incorrect_lists);
    for list in incorrect_lists {
        rtn += list[list.len()/2];
    }
    Some(rtn)
}

fn fix_incorrect_lists(order: & Vec<(i32,i32)>,incorrect_lists: &mut Vec<Vec<i32>>) {

    for list in incorrect_lists {
       'pages: for i in 0..list.len() {
            for j in i + 1..list.len() {
                if !check_if_tuple_correct(&order,(list[i],list[j])){
                    list.swap(i.try_into().unwrap(),j.try_into().unwrap());
                }
            }
        }
    }
}
fn get_correct_order(order: &Vec<(i32,i32)>, lists: &Vec<Vec<i32>>) -> (Vec<Vec<i32>>,Vec<Vec<i32>>) {
    let mut correct: Vec<Vec<i32>> = Vec::new();
    let mut incorrect: Vec<Vec<i32>> = Vec::new();
    for list in lists {
        let mut success = true;
        'pages: for i in 0..list.len() {
            for j in i + 1..list.len() {
                if !check_if_tuple_correct(&order,(list[i],list[j])){
                    success = false;
                    incorrect.push(list.to_vec());
                    break 'pages;
                }
            }
        }
        if success {
            correct.push(list.to_vec());
        }
    }
    (correct,incorrect)
}

fn check_if_tuple_correct(order: &Vec<(i32,i32)>, target: (i32,i32)) -> bool {
    if order.contains(&(target.1, target.0)) {
        return false;
    }
    true
}
