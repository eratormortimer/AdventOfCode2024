use std::io;

fn wait_for_enter() {
    let mut input = String::new();
    println!("Press Enter to continue...");
    io::stdin().read_line(&mut input).expect("Failed to read line");
}

pub fn get_xmas(lists: Vec<Vec<char>>) -> Option<i32> {
    let mut rtn: i32 = 0;
    let target = vec!['X', 'M', 'A', 'S'];
    let directions = [
        (-1, -1), (-1, 0), (-1, 1), // Top row
        (0, -1),          (0, 1),   // Middle row
        (1, -1), (1, 0), (1, 1),   // Bottom row
    ];

    fn search(
        current_depth: usize,
        position: (i32, i32),
        lists: &Vec<Vec<char>>,
        target: &Vec<char>,
        direction: (i32,i32),
    ) -> i32 {
        if let Some(row) = lists.get(position.0 as usize) {
            if let Some(value) = row.get(position.1 as usize) {
                if *value == target[current_depth] {
                    if current_depth == target.len() - 1 {
                        //println!("current_depth: {:?}, position: {:?}",current_depth,position);
                        //wait_for_enter();
                        return 1;
                    } else {
                        let mut search_rtn = 0;
                        //for (dr, dc) in directions {
                            search_rtn += search(
                                current_depth + 1,
                                (position.0 + direction.0, position.1 + direction.1),
                                lists,
                                target,
                                direction,
                            );
                        //}
                        return search_rtn;
                    }
                } else {
                    return 0;
                }
            }
        }
        0
    }

    for (pos_x, vec_x) in lists.iter().enumerate() {
        for (pos_y, _vec_y) in vec_x.iter().enumerate() {
            println!("x: {:?}, y: {:?}",pos_x,pos_y);
            for direction in directions {
                rtn += search(0, (pos_x as i32, pos_y as i32), &lists, &target, direction);
            }
        }
    }
    Some(rtn)
}

pub fn get_x_mas(lists: Vec<Vec<char>>) -> Option<i32> {
    let mut rtn: i32 = 0;
    let target = vec!['X', 'M', 'A', 'S'];
    let directions = [
        (-1, -1),(-1, 1), // Top row
        (1, -1),(1, 1),   // Bottom row
    ];

    fn search(
        lists: &Vec<Vec<char>>,
        position: (i32, i32),
        directions: [(i32,i32);4]
    ) -> i32 {
        let mut target: Vec<char> = Vec::new();
        for direction in directions{
            let pos_0 = position.0 + direction.0;
            let pos_1 = position.1 + direction.1;
            if let Some(row) = lists.get(pos_0 as usize) {
                if let Some(value) = row.get(pos_1 as usize) {
                    target.push(*value)
                }
            }
        }
        if target.len() == 4 {
            match format!("{}{}{}{}", target[0], target[3], target[1], target[2]) {
                val if val == "MSMS".to_string() => return 1,
                val if val == "MSSM".to_string() => return 1,
                val if val == "SMMS".to_string() => return 1,
                val if val == "SMSM".to_string() => return 1,
                _ => return 0,
            }
        }
        0
    }

    for (pos_x, vec_x) in lists.iter().enumerate() {
        for (pos_y, vec_y) in vec_x.iter().enumerate() {
            if *vec_y == 'A' {
                rtn += search(&lists,(pos_x as i32, pos_y as i32),directions)
            }
           
        }
    }
    Some(rtn)
}