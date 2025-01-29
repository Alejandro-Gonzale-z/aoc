use std::fs::read_to_string;

fn main() {
    let file_content = read_to_string("./src/day5.txt").unwrap();
    let test_content = read_to_string("./src/day5_test_input.txt").unwrap();
    
    let content_split: Vec<&str> = file_content.split("\n\n").collect();
    let rules = content_split.get(0).unwrap().trim();
    let page_list = content_split.get(1).unwrap().trim();

    let mut rules_vec: Vec<[i32; 2]> = vec!();
    let mut page_list_vec: Vec<Vec<i32>> = vec!();

    let mut flag: bool = true;
    let mut total: i32 = 0;

    // store all of the rules in the initial lines into a vector of tuples
    for line in rules.lines() {
        let split: Vec<&str> = line.split("|").collect();
        let x: i32 = split.get(0).unwrap().trim().parse().unwrap();
        let y: i32 = split.get(1).unwrap().trim().parse().unwrap();
        rules_vec.push([x,y]);

    }

    // store the list of pages into a num vector
    for line in page_list.lines() {
        let split: Vec<&str> = line.split(",").collect();
        let mut row: Vec<i32> = Vec::new();

        for value in split {
            let x: i32 = value.parse().unwrap();
            row.push(x);
        }
        page_list_vec.push(row);
    }

    // for each item in the page list
    // find all the secondary pairs for that value,
    // then make sure that each following element is a pair of that item
    for line in page_list_vec {
        for index in 0..line.len() {
            let pair_values: Vec<i32> = pair_value_vector(line[index], &rules_vec);
            if !vecs_have_same_elements(index, &line, &pair_values) {
                flag = false;
            }
        }
        if flag {
            let middle_index = line.len() / 2;
            println!("{}",middle_index);
            let middle_item = line[middle_index];
            total += middle_item;
            println!("{:?} is good value is {}",line,middle_item);
        } else {
            flag = true;
        }
    }

    println!("total: {}", total);


}

// finds all the pairs of item x and returns them as an array
// this will only return which are intended to proceed X
fn pair_value_vector(x: i32, list: &Vec<[i32; 2]>) -> Vec<i32>{
    let mut pair_values: Vec<i32> = Vec::new();
    for arr in list {
        if arr[0] == x {
            pair_values.push(arr[1]);
        }
    }
    return pair_values;
}

// i is the index of the element being inspected
// vec1 is the current line being inspected
// vec2 is the list of pairs for the value X
fn vecs_have_same_elements(x: usize, vec1: &Vec<i32>, vec2: &Vec<i32>) -> bool{
    // make clones of vec1, the current line so that removing the x value does not affect original vector
    let mut vec1_clone = vec1.clone();
    // remove all elements in the line from 0 to the index of current element being inspected
    vec1_clone.drain(0..x+1);
    //for every value in the line, make sure it is a pair of x
    for value in vec1_clone {
        if !vec2.contains(&value) {
            return false;
        }
    }

    return true;
}