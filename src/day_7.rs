use std::io;
fn wait_for_enter() {
    let mut input = String::new();
    println!("Press Enter to continue...");
    io::stdin().read_line(&mut input).expect("Failed to read line");
}
pub fn calc_muts(sums: Vec<(i64,Vec<i64>)>) -> Option<i64> {
    let mut rtn: i64 = 0;
    for (single_sum,summands) in sums {
        if correct_sum(single_sum,summands) {
            rtn = rtn+single_sum;
        }  
    }
    Some(rtn)
}

fn correct_sum(single_sum: i64, mut summands: Vec<i64>) -> bool {
    fn search(
        mut rest_summands: Vec<i64>,
    ) -> Vec<i64> {
        let x = rest_summands.remove(0);
        let mut rtn: Vec<i64> = Vec::new();
        if rest_summands.len() == 0{
            rtn.push(x);
            return rtn;
        }
        let rest = search(rest_summands);
        rtn.extend(rest.iter().map(|y| y*x));
        rtn.extend(rest.iter().map(|y| y+x));
        rtn.extend(rest.iter().map(|&y| {
            y * 10_i64.pow(x.abs().to_string().len() as u32) + x
        }));
        return rtn;
    }

    summands.reverse();
    let sums = search(summands);
    if sums.contains(&single_sum) {
        return true;
    }
    return false;
}
