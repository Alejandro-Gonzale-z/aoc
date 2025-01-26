use std::fs::read_to_string;
use regex::Regex;

fn main() {
    let file_content: String = read_to_string("./src/day3.txt").unwrap();
    let re_mul_statement = Regex::new(r"(mul\([0-9]{1,3},[0-9]{1,3}\))").unwrap();
    let mut total = 0;
    
    let captures = re_mul_statement.captures_iter(&file_content);
    for cap in captures {
        let re_numbers: Regex = Regex::new(r"([0-9]{1,3})").unwrap();
       
        let nums: Vec<&str> = re_numbers.find_iter(&cap.get(0).unwrap().as_str()).map(|m| m.as_str()).collect();
        let x: i32 = nums[0].parse().unwrap();
        let y: i32 = nums[1].parse().unwrap();
        total += y * x;
    }

    println!("{}", total);
}
