use std::io;
fn wait_for_enter() {
    let mut input = String::new();
    println!("Press Enter to continue...");
    io::stdin().read_line(&mut input).expect("Failed to read line");
}
pub fn get_xmas(lists:Vec<Vec<String>>) -> Option<i32> {
    let mut rtn: i32 = 0;
    fn search(current_depth: i32, position: (i32,i32)) -> i32 {
        if current_depth == 4:
            if let Some(row) = list.get(position.0) && let Some(value) = row.get(position.1) && value == "S" {
                return 1;
            }
        match
        left = search(current_depth+1)
    }
    Some(rtn)
}

