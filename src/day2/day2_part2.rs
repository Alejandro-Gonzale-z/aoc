use std::fs::read_to_string;

fn main() {
    let file_path = "./src/day2.txt";
    let content = read_to_string(file_path).expect("Failed to read file");
    
    let total_safe = content
        .lines()
        .map(|line| verify(line, -1))
        .filter(|&is_safe| is_safe)
        .count();

    println!("Total safe: {}", total_safe);
}

fn verify(line: &str, exclude_idx: isize) -> bool {
    let mut nums: Vec<u32> = line
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    if exclude_idx >= 0 {
        nums.remove(exclude_idx as usize);
    }

    let is_increasing = nums[1] > nums[0];
    let mut is_safe = true;

    for i in 1..nums.len() {
        let diff = if is_increasing {
            nums[i] as isize - nums[i - 1] as isize
        } else {
            nums[i - 1] as isize - nums[i] as isize
        };
        is_safe &= diff >= 1 && diff <= 3;
    }

    if exclude_idx == -1 && !is_safe {
        for i in 0..nums.len() {
            if verify(line, i as isize) {
                return true;
            }
        }
    }

    is_safe
}