use std::io;
use std::collections::HashSet;
use std::convert::TryFrom;

const UP: (i32, i32) = (-1, 0);
const DOWN: (i32, i32) = (1, 0);
const LEFT: (i32, i32) = (0, -1);
const RIGHT: (i32, i32) = (0, 1);


fn add_tuples(t1: (usize, usize), t2: (i32, i32)) -> (usize, usize) {
    // Convert usize to i64 to avoid mismatched types
    let (x,y) = (t1.0 as i32 + t2.0, t1.1 as i32 + t2.1);
    (x as usize, y as usize)
}

fn wait_for_enter() {
    let mut input = String::new();
    println!("Press Enter to continue...");
    io::stdin().read_line(&mut input).expect("Failed to read line");
}
pub fn trailhead_score(hiking_map: Vec<Vec<i64>>) -> Option<usize> {
    fn search_path(hiking_map: &Vec<Vec<i64>>, startpoint: (usize,usize), oldpoint: (usize,usize) ) -> Option<HashSet<(usize,usize)>> {
        if hiking_map[startpoint.0][startpoint.1] == 9 {
            let mut rtn: HashSet<(usize,usize)> = HashSet::new();
            rtn.insert(startpoint);
            return Some(rtn);
        }
        if hiking_map[startpoint.0][startpoint.1] != 0 && hiking_map[startpoint.0][startpoint.1] - hiking_map[oldpoint.0][oldpoint.1] != 1 {
            return None;
        }
        let mut rtn: HashSet<(usize,usize)> = HashSet::new();
        if startpoint.0 > 0 {
            let newpoint = add_tuples(startpoint,UP);
            if hiking_map[newpoint.0][newpoint.1] - hiking_map[startpoint.0][startpoint.1] == 1 {
                if let Some(result) = search_path(&hiking_map,newpoint,startpoint) {
                    rtn.extend(result);
                }
            }
        }
        if startpoint.0 < hiking_map.len()-1 {
            let newpoint = add_tuples(startpoint,DOWN);
            if hiking_map[newpoint.0][newpoint.1] - hiking_map[startpoint.0][startpoint.1] == 1 {
                if let Some(result) = search_path(&hiking_map,newpoint,startpoint) {
                    rtn.extend(result);
                }
            }
        }
        if startpoint.1 > 0 {
            let newpoint = add_tuples(startpoint,LEFT);
            if hiking_map[newpoint.0][newpoint.1] - hiking_map[startpoint.0][startpoint.1] == 1{
                if let Some(result) = search_path(&hiking_map,newpoint,startpoint) {
                    rtn.extend(result);
                }
            }
        }
        if startpoint.1 < hiking_map[0].len()-1 {
            let newpoint = add_tuples(startpoint,RIGHT);
            if hiking_map[newpoint.0][newpoint.1] - hiking_map[startpoint.0][startpoint.1] == 1 {
                if let Some(result) = search_path(&hiking_map,newpoint,startpoint) {
                    rtn.extend(result);
                }
            }
        }
        Some(rtn)
    }
    let starts = get_starts(&hiking_map);
    let mut rtn: usize = 0;
    for start in starts {
        if let Some(result) = search_path(&hiking_map,start,start) {
            rtn += result.len();
        }
    }
    Some(rtn)
}

pub fn trailhead_score_part2(hiking_map: Vec<Vec<i64>>) -> Option<usize> {
    fn search_path(hiking_map: &Vec<Vec<i64>>, startpoint: (usize,usize), path_until_now: Vec<(usize,usize)> ) -> Option<HashSet<Vec<(usize,usize)>>> {
        let mut path_until_now = path_until_now.clone();
        path_until_now.push(startpoint);
        if hiking_map[startpoint.0][startpoint.1] == 9 {
            let mut rtn: HashSet<Vec<(usize,usize)>> = HashSet::new();
            rtn.insert(path_until_now);
            return Some(rtn);
        }
        
        let mut rtn: HashSet<Vec<(usize,usize)>> = HashSet::new();
        if startpoint.0 > 0 {
            let newpoint = add_tuples(startpoint,UP);
            if hiking_map[newpoint.0][newpoint.1] - hiking_map[startpoint.0][startpoint.1] == 1 {
                if let Some(result) = search_path(&hiking_map,newpoint,path_until_now.clone()) {
                    rtn.extend(result);
                }
            }
        }
        if startpoint.0 < hiking_map.len()-1 {
            let newpoint = add_tuples(startpoint,DOWN);
            if hiking_map[newpoint.0][newpoint.1] - hiking_map[startpoint.0][startpoint.1] == 1 {
                if let Some(result) = search_path(&hiking_map,newpoint,path_until_now.clone()) {
                    rtn.extend(result);
                }
            }
        }
        if startpoint.1 > 0 {
            let newpoint = add_tuples(startpoint,LEFT);
            if hiking_map[newpoint.0][newpoint.1] - hiking_map[startpoint.0][startpoint.1] == 1{
                if let Some(result) = search_path(&hiking_map,newpoint,path_until_now.clone()) {
                    rtn.extend(result);
                }
            }
        }
        if startpoint.1 < hiking_map[0].len()-1 {
            let newpoint = add_tuples(startpoint,RIGHT);
            if hiking_map[newpoint.0][newpoint.1] - hiking_map[startpoint.0][startpoint.1] == 1 {
                if let Some(result) = search_path(&hiking_map,newpoint,path_until_now.clone()) {
                    rtn.extend(result);
                }
            }
        }
        Some(rtn)
    }
    let starts = get_starts(&hiking_map);
    let mut rtn: usize = 0;
    for start in starts {
        let path: Vec<(usize,usize)> = Vec::new();
        if let Some(result) = search_path(&hiking_map,start,path) {
            rtn += result.len();
        }
    }
    Some(rtn)
}

fn get_starts(hiking_map: &Vec<Vec<i64>>) -> Vec<(usize,usize)> {
    let mut rtn: Vec<(usize,usize)> = Vec::new();
    for (pos_x,line) in hiking_map.iter().enumerate() {
        for (pos_y,element) in line.iter().enumerate() {
            if *element == 0 {
                rtn.push((pos_x,pos_y));
            }
        }
    }
    rtn
}

fn get_endpoints(hiking_map: &Vec<Vec<i64>>) -> Vec<(usize,usize)> {
    let mut rtn: Vec<(usize,usize)> = Vec::new();
    for (pos_x,line) in hiking_map.iter().enumerate() {
        for (pos_y,element) in line.iter().enumerate() {
            if *element == 9 {
                rtn.push((pos_x,pos_y));
            }
        }
    }
    rtn
}