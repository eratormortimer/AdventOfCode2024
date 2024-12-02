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
   
    let mut safe = 0;
    for list in lists {
        let mut list: Vec<i32> = list
        .into_iter()
        .map(|s| s.parse::<i32>().unwrap_or(0)) // Parse each string to i32, defaulting to 0 if it fails
        .collect();
        let mut successfull = 2;
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
                        successfull = successfull -1;
                        if successfull == 0 {
                            break;
                        } else {
                            continue;
                        }
                    }        
                }
            }
            if (number - old_num_unwr).abs() > 3 || (number - old_num_unwr).abs() == 0 {
                successfull = successfull -1;
                if successfull == 0 {
                    break;
                } else {
                    continue;
                }
            }
            old_num = Some(number);

        }
        if successfull>0 {
            safe = safe +1;
        }
    }
    Some(safe)
}