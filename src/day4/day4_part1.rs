use std::fs::read_to_string;

fn main() {
    let file_content = read_to_string("./src/day4.txt").unwrap();

    let file_vec: Vec<Vec<char>> = file_content
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let mut total = 0;

    for i in 0..file_vec.len(){
        let elements_in_row = file_vec[i].len();
        let num_columns = file_vec.len();
        for j in 0..elements_in_row{
            
            if file_vec[i][j] == 'X' {
                // check horizontal 
                if j <= elements_in_row - 4 {
                    let sliced_string: String = file_vec[i][j..j+4].iter().collect();
                    if sliced_string == "XMAS" {
                        total += 1;
                    }
                }
                //check vertical
                if i <= num_columns - 4 {
                    let sliced_string: String = file_vec[i..i+4].iter().map(|row| row[j]).collect();
                    if sliced_string == "XMAS" {
                        total += 1;
                    }
                }
                // check diagonal right
                if i <= num_columns - 4 && j <= elements_in_row - 4 {
                    let sliced_string: String = (0..4).map(|offset| file_vec[i+offset][j+offset]).collect();
                    if sliced_string == "XMAS" {
                        total +=1;
                    }
                }
                // check diagonal left
                if i <= num_columns - 4 && j >= 3 {
                    let sliced_string: String = (0..4).map(|offset| file_vec[i+offset][j-offset]).collect();
                    if sliced_string == "XMAS" {
                        total +=1;
                    }
                }
            }

            if file_vec[i][j] == 'S' {
                // check horizontal backwards
                if j <= elements_in_row - 4 {
                    let sliced_string: String = file_vec[i][j..j+4].iter().collect();
                    if sliced_string == "SAMX" {
                        total += 1;
                    }
                }
                // check vertical backwards
                if i <= num_columns - 4 {
                    let sliced_string: String = file_vec[i..i+4].iter().map(|row| row[j]).collect();
                    if sliced_string == "SAMX" {
                        total += 1;
                    }
                }
                // check diagonal right backwards
                if i <= num_columns - 4 && j <= elements_in_row - 4 {
                    let sliced_string: String = (0..4).map(|offset| file_vec[i+offset][j+offset]).collect();
                    if sliced_string == "SAMX" {
                        total +=1;
                    }
                }
                // check diagonal left backwards
                if i <= num_columns - 4 && j >= 3 {
                    let sliced_string: String = (0..4).map(|offset| file_vec[i+offset][j-offset]).collect();
                    if sliced_string == "SAMX" {
                        total +=1;
                    }
                }

            }
        }
    }

    println!("{}", total);
}