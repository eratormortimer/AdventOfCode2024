use std::io;
fn wait_for_enter() {
    let mut input = String::new();
    println!("Press Enter to continue...");
    io::stdin().read_line(&mut input).expect("Failed to read line");
}
pub fn calc_safe_amount(lists:Vec<Vec<String>>) -> Option<i32> {
     // Convert strings to integers
    
    let mut safe = 0;
    for list in lists {
        let mut list: Vec<i32> = list
        .into_iter()
        .map(|s| s.parse::<i32>().unwrap_or(0)) // Parse each string to i32, defaulting to 0 if it fails
        .collect();
        let mut successfull = true;
        let mut direction_up: Option<bool> = None;
        let mut old_num: Option<i32> = None;
        for number in list {
            if old_num.is_none() {
                old_num = Some(number);
                continue;
            }
            let old_num_unwr = old_num.unwrap_or(0);
            let new_direction = number>old_num_unwr;
            match direction_up {
                None => {
                    direction_up = Some(new_direction)
                }
                Some(old_direction) => {
                    if old_direction != new_direction {
                        successfull = false;
                        break;
                    }        
                }
            }
            if (number - old_num_unwr).abs() > 3 || (number - old_num_unwr).abs() == 0 {
                successfull = false;
                break;
            }
            old_num = Some(number);

        }
        if successfull {
            safe = safe +1;
        }
    }
    Some(safe)

}

pub fn calc_safe_amount_with_level(lists:Vec<Vec<String>>) -> Option<i32> {
    // Convert strings to integers
    fn is_list_safe(list: &Vec<i32>) -> (bool,usize) {
        let mut successfull = true;
        let mut broken_position: usize = 0;
        let mut direction_up: Option<bool> = None;
        let mut old_num: Option<i32> = None;
        for (index,number) in list.into_iter().copied().enumerate() {
            if old_num.is_none() {
                old_num = Some(number);
                continue;
            }
            let old_num_unwr = old_num.unwrap_or(0);
            let new_direction = number>old_num_unwr;
            match direction_up {
                None => {
                    direction_up = Some(new_direction)
                }
                Some(old_direction) => {
                    if old_direction != new_direction {
                        successfull = false;
                        broken_position = index-1;
                        break;
                    }        
                }
            }
            if (number - old_num_unwr).abs() > 3 || (number - old_num_unwr).abs() == 0 {
                successfull = false;
                broken_position = index-1;
                break;
            }
            old_num = Some(number);

        }
        (successfull,broken_position)
    } 
    let mut safe = 0;
    for list in lists {
        let mut list: Vec<i32> = list
        .into_iter()
        .map(|s| s.parse::<i32>().unwrap_or(0)) // Parse each string to i32, defaulting to 0 if it fails
        .collect();
        //println!("The list: {:?}.",list);
        let (mut successfull,mut broken_position) = is_list_safe(&list);
        if !successfull {
            let mut list_bk = list.clone();
            let mut list_bk_2 = list.clone();
            let mut list_bk_3 = list.clone();
            list.remove(broken_position);
            list_bk_3.remove(broken_position+1);
            (successfull,broken_position) = is_list_safe(&list);
            // Special case if we broke in the beginning
            if !successfull {
                list_bk.remove(0);
                (successfull,broken_position) = is_list_safe(&list_bk);
            }
            if !successfull {
                list_bk_2.remove(list_bk_2.len()-1);
                (successfull,broken_position) = is_list_safe(&list_bk_2);
            }
            if !successfull {
                (successfull,broken_position) = is_list_safe(&list_bk_3);
            }
            //println!("The middle: {:?} {:?} {:?}.",list,successfull,broken_position);
        }
        if successfull {
            safe = safe+1;
        }
        //println!("The end: {:?} {:?} {:?}.",list,successfull,broken_position);
        //wait_for_enter();
    }
    Some(safe)
}