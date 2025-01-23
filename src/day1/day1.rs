use std::cmp::Ordering;
use std::fs::read_to_string;

fn main() {
    let file_path = String::from("./src/day1.txt");
    let mut list_one: Vec<u32> = Vec::new();
    let mut list_two: Vec<u32> = Vec::new();
    let content = read_to_string(file_path).unwrap();

    for line in content.lines() {
        let parts: Vec<&str> = line.split("   ").collect();

        let item1: u32 = parts[0].parse().unwrap();
        let item2: u32 = parts[1].parse().unwrap();
        
        list_one.push(item1);
        list_two.push(item2);
    }        
    list_one.sort();
    list_two.sort();

    let total = total_distance(&list_one, &list_two);
    println!("total distance: {}",total);

    let similarity_score = similarity_score(&list_one, &list_two);
    println!("similarity score: {}", similarity_score);

}

fn abs_distance(item1: u32, item2: u32) -> u32 {
    match item1.cmp(&item2) {
        Ordering::Less => item2 - item1,
        Ordering::Greater => item1 - item2,
        Ordering::Equal => 0
    }
}

fn total_distance(list_one: &Vec<u32>, list_two: &Vec<u32>) -> u32 {
    let mut total: u32 = 0;
    for (i, item) in list_one.iter().enumerate() {
        let distance = abs_distance(*item,list_two[i]);
        total += distance;
        println!("{} - {} = {}",item, list_two[i], distance);
    }
   total
}

fn num_occurences(num: u32, list: &Vec<u32>) -> u32 {
    let mut occurences = 0;
    for item in list.iter() {
        if *item > num {
            return occurences;
        }        
        if *item == num {
            occurences += 1;
        }
    }
    occurences
}

fn similarity_score(list_one: &Vec<u32>, list_two: &Vec<u32>) -> u32 {
    let mut score = 0;
    for item in list_one.iter() {
        let occurences = num_occurences(*item, list_two);
        score += *item * occurences;
        // println!("item: {}, occurences: {}", *item,occurences);
    }
    score
}