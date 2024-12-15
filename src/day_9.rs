use std::io;

fn wait_for_enter() {
    let mut input = String::new();
    println!("Press Enter to continue...");
    io::stdin().read_line(&mut input).expect("Failed to read line");
}
pub fn defragment_disk(disk: Vec<i64>) -> Option<i64> {
    let mut pointer_left_empty: i64;
    let mut pointer_right_data: i64;
    let mut disk = disk.clone();
    pointer_left_empty = update_pointer_left(&disk);
    pointer_right_data = update_pointer_right(&disk,disk.len() as i64);
    while pointer_left_empty < pointer_right_data {
        disk[pointer_left_empty as usize] = disk.remove(pointer_right_data.try_into().unwrap());
        pointer_left_empty = update_pointer_left(&disk);
        pointer_right_data = update_pointer_right(&disk,pointer_right_data);
    }
    //println!("The defragmented disk: {:?}.",disk);
    Some(calculate_disk_checksum(disk))
}

pub fn defragment_disk_in_files(disk: Vec<i64>) -> Option<i64> {
    let mut pointer_left_empty: i64;
    let mut pointer_right_data: i64;
    let mut disk = disk.clone();
    pointer_left_empty = update_pointer_left(&disk);
    pointer_right_data = update_pointer_right(&disk,disk.len() as i64);
    //println!("pointer_left: {:?}    pointer_right: {:?}.",pointer_left_empty,pointer_right_data);
    while pointer_right_data > 0 {
        if pointer_left_empty < pointer_right_data {
            let element_size = get_element_size(&disk,pointer_right_data);
            //println!("left_empty {:?}    element size: {:?}.",get_empty_size(&disk,pointer_left_empty),element_size);
            if get_empty_size(&disk,pointer_left_empty) >= element_size {
                // Element fits in the space and can be moved
                for i in 0..element_size {
                    disk[pointer_left_empty as usize] = disk[pointer_right_data as usize];
                    disk[pointer_right_data as usize]=-1;
                    pointer_left_empty+=1;
                    pointer_right_data-=1;
                }
                pointer_left_empty = update_pointer_left(&disk);
                if disk[pointer_right_data as usize] ==-1 {
                    pointer_right_data = update_pointer_right(&disk,pointer_right_data);
                }
            } else {
                // we look for the next free space
                pointer_left_empty = get_next_empty(&disk,pointer_left_empty);
            }
        } else {
            // element has no empty space big enough somewhere, we skip it
            pointer_right_data = get_next_file(&disk,pointer_right_data);
            pointer_left_empty = update_pointer_left(&disk);
        }
        //println!("The defragmented disk: {:?}.",disk);
        //println!("pointer_left: {:?}    pointer_right: {:?}.",pointer_left_empty,pointer_right_data);
    }
    //println!("The defragmented disk: {:?}.",disk);
    Some(calculate_disk_checksum(disk))
}

fn update_pointer_left(disk: &Vec<i64>) -> i64 {
    for (id,disk_data) in disk.iter().enumerate() {
        if *disk_data == -1 {
            return id.try_into().unwrap();
        }
    }
    -9
}

fn update_pointer_right(disk: &Vec<i64>,pointer_right_data: i64) -> i64 {
    for (id,disk_data) in disk[..pointer_right_data as usize].iter().enumerate().rev() {
        if *disk_data != -1 {
            return id.try_into().unwrap()
        }
    }
    -9
}

fn calculate_disk_checksum(disk: Vec<i64>) -> i64 {
    let mut rtn: i64 = 0;
    for (id,disk_data) in disk.iter().enumerate() {
        if *disk_data == -1 {
            continue;
        }
        rtn += id as i64 * disk_data;
    }
    rtn 
}

fn get_empty_size(disk: &Vec<i64>, pointer_left_empty: i64) -> i64{
    let mut rtn =0;
    for (id,disk_data) in disk[pointer_left_empty as usize ..].iter().enumerate() {
        if *disk_data == -1 {
            rtn += 1;
        } else {
            break;
        }
    }
    rtn
}
fn get_element_size(disk: &Vec<i64>, pointer_right_data: i64) -> i64{
    let mut rtn =1;
    for (id,disk_data) in disk[..pointer_right_data as usize].iter().enumerate().rev() {
        //println!("id: {:?}, data: {:?}",id,disk_data); 
        if *disk_data == disk[pointer_right_data as usize] {
            rtn +=1;
        }else {
            break;
        }
    }
    rtn
}   
fn get_next_empty(disk: &Vec<i64>, pointer_left_empty: i64) -> i64{
    let mut rtn = pointer_left_empty;
    let mut next = false;
    for (id,disk_data) in disk[pointer_left_empty as usize..].iter().enumerate() {
        if *disk_data == -1 {
            if next {
                break;
            }
            rtn +=1;
        } else {
            rtn +=1;
            next = true;
        }
    }
    rtn
}
fn get_next_file(disk: &Vec<i64>, pointer_right_data: i64) -> i64{
    let mut rtn = pointer_right_data;
    for (id,disk_data) in disk[..pointer_right_data as usize].iter().enumerate().rev() {
        rtn -= 1;
        if *disk_data != -1 && *disk_data != disk[pointer_right_data as usize] {
            break;
        }
    }
    rtn as i64
}