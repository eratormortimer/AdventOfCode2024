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
        directions: &[(i32, i32)],
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
                        for (dr, dc) in directions {
                            search_rtn += search(
                                current_depth + 1,
                                (position.0 + dr, position.1 + dc),
                                lists,
                                target,
                                directions,
                            );
                        }
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
            rtn += search(0, (pos_x as i32, pos_y as i32), &lists, &target, &directions);
        }
    }
    Some(rtn)
}