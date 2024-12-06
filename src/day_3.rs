use std::io;
fn wait_for_enter() {
    let mut input = String::new();
    println!("Press Enter to continue...");
    io::stdin().read_line(&mut input).expect("Failed to read line");
}
pub fn calc_muts(tuples: Vec<(i32,i32)>) -> Option<i32> {
    let mut rtn: i32 = 0;
    for tuple in tuples {
        println!("{:?}",tuple);
        rtn += tuple.0*tuple.1;
    }
    Some(rtn)
}

