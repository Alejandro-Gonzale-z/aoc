use std::fs::read_to_string;

fn main() {
    let file_content = read_to_string("./src/day5.txt").unwrap();
    let test_content = read_to_string("./src/day5_test_input.txt").unwrap();

    let content_split: Vec<&str> = file_content.split("\n\n").collect();
    let rules = content_split.get(0).unwrap().trim();
    let page_list = content_split.get(1).unwrap().trim();

    let mut rules_vec: Vec<[i32; 2]> = vec![];
    let mut page_list_vec: Vec<Vec<i32>> = vec![];

    let mut flag: bool = true;
    let mut total: i32 = 0;

    // store all of the rules in the initial lines into a vector of tuples
    for line in rules.lines() {
        let split: Vec<&str> = line.split("|").collect();
        let x: i32 = split.get(0).unwrap().trim().parse().unwrap();
        let y: i32 = split.get(1).unwrap().trim().parse().unwrap();
        rules_vec.push([x, y]);
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
        if !flag {
            // PART TWO CODE TO FIX BAD LINES
            // find the paired value for each element in the line and add to a new Vector<original value, number of pairs>
            // then sort the vector in descending order according to number of pairs
            println!("{:?} is bad", line);
            
            // create a new vector the first element is the number in the line, and the second element is the number of pairs
            let mut tuple_vec: Vec<(i32, i32)> = Vec::new();
            for index in 0..line.len() {
                let pair_values: Vec<i32> = pair_value_vector(line[index], &rules_vec);
                let num_pairs_in_line: i32 = num_shared_elements(&line, &pair_values);
                tuple_vec.push((line[index], num_pairs_in_line));
            }
            // sort this tuple in descending order according to the second value
            tuple_vec.sort_by(|a, b| b.1.cmp(&a.1));

            // find the value in the middle and add it to the total
            let middle_index = tuple_vec.len() / 2;
            let middle_value = tuple_vec[middle_index].0;
            println!("{}", middle_value);
            total += middle_value;

            flag = true;
        }
    }

    println!("total: {}", total);
}

// finds all the pairs of item x and returns them as an array
// this will only return which are intended to proceed X
fn pair_value_vector(x: i32, list: &Vec<[i32; 2]>) -> Vec<i32> {
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
fn vecs_have_same_elements(x: usize, vec1: &Vec<i32>, vec2: &Vec<i32>) -> bool {
    // make clones of vec1, the current line so that removing the x value does not affect original vector
    let mut vec1_clone = vec1.clone();
    // remove all elements in the line from 0 to the index of current element being inspected
    vec1_clone.drain(0..x + 1);
    //for every value in the line, make sure it is a pair of x
    for value in vec1_clone {
        if !vec2.contains(&value) {
            return false;
        }
    }

    return true;
}

// function to return the number of pairs an element has that are in the current line
// vec1 is the current line being inspected
// vec2 is the list of pairs for the value X
fn num_shared_elements(vec1: &Vec<i32>, vec2: &Vec<i32>) -> i32 {
    let mut total = 0;

    for value in vec2 {
        if vec1.contains(value) {
            total += 1;
        }
    }

    return total;
}
