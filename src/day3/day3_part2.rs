use regex::Regex;
use std::fs::read_to_string;

fn main() {
    let file_content: String = read_to_string("./src/day3.txt").unwrap();
    // test input from AOC day 3 page => expected output is 48
    // let test_content: String = String::from("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))");
    
    // use a regex statement to capture either a mul do or dont statement
    let re: Regex = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)|do\(\)|don't\(\)").unwrap();
    // collect all the captured statements into a Vec<&str> variable
    let captures: Vec<&str> = re.find_iter(&file_content).map(|m| m.as_str()).collect();

    let mut total = 0;
    let mut flag = true;

    for cap in captures {
        match cap {
            "do()" => flag = true,
            "don't()" => flag = false,
            _ => {
                if flag {
                    // use regex to extract the numbers from the mul statement
                    let re_numbers: Regex = Regex::new(r"([0-9]{1,3})").unwrap();
                    let nums: Vec<&str> = re_numbers.find_iter(cap).map(|m| m.as_str()).collect();
                    let x: i32 = nums[0].parse().unwrap();
                    let y: i32 = nums[1].parse().unwrap();
                    total += x * y;
                }
            }            
        }
    }

    println!("{}",total);
}