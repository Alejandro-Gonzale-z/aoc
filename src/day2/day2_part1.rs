use std::{cmp::Ordering, fs::read_to_string};

fn main() {
    let file_path: String = String::from("./src/day2.txt");
    let content: String = read_to_string(file_path).unwrap();
    let mut total_safe = 0;
    
    for line in content.lines() {
        let mut num_vec: Vec<u32> = Vec::new();
        let parts: Vec<&str> = line.split_whitespace().collect();
        for num in parts {
            let item: u32 = num.parse().unwrap();
            num_vec.push(item);
        }
        total_safe += line_is_safe(&mut num_vec);
    }

    println!("total_safe: {}", total_safe);
}

// this function will return either 0 or 1
// 1 indicates that the line is safe, 0 indicates it is NOT safe
// "Safe" is dependent on two conditions
// - The levels are either all increasing or all decreasing.
// - Any two adjacent levels differ by at least one and at most three.
fn line_is_safe(list: &Vec<u32>) -> u32 {
    let mut is_safe = 0;
    let decreasing = line_is_decreasing(list);
    let increasing = line_is_increasing(list);
    if (decreasing || increasing) && line_difference_is_ok(list) {
        is_safe += 1
    }
    is_safe
}

fn line_difference_is_ok(list: &Vec<u32>) -> bool {
    for i in 0..list.len() - 1 {
        let diff = abs_diff(list[i], list[i + 1]);
        if diff < 1 || diff > 3 {
            return false;
        }
    }
    return true;
}

fn abs_diff(item1: u32, item2: u32) -> u32 {
    match item1.cmp(&item2) {
        Ordering::Less => item2 - item1,
        Ordering::Greater => item1 - item2,
        Ordering::Equal => 0,
    }
}

fn line_is_decreasing(list: &Vec<u32>) -> bool {
    for i in 0..list.len() - 1 {
        if list[i] <= list[i + 1] {
            return false;
        }
    }
    return true;
}

fn line_is_increasing(list: &Vec<u32>) -> bool {
    for i in 0..list.len() - 1 {
        if list[i] >= list[i + 1] {
            return false;
        }
    }
    return true;
}
