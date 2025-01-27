use std::fs::read_to_string;

fn main() {
    let file_content = read_to_string("./src/day4.txt").unwrap();
    
    let file_vec: Vec<Vec<char>> = file_content
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let mut total = 0;

    for i in 0..file_vec.len() - 2 {
        let elements_in_row = file_vec[i].len();

        for j in 0..elements_in_row - 2 {
            // mas mas check
            if file_vec[i][j] == 'M'
                && file_vec[i][j + 2] == 'M'
                && file_vec[i + 1][j + 1] == 'A'
                && file_vec[i + 2][j] == 'S'
                && file_vec[i + 2][j + 2] == 'S'
            {
                total += 1;
            }
            // mas sam check
            if file_vec[i][j] == 'M'
                && file_vec[i][j+2] == 'S'
                && file_vec[i+1][j+1] == 'A'
                && file_vec[i+2][j] == 'M'
                && file_vec[i+2][j+2] == 'S' 
            {
                total += 1;
            }
            // sam sam check
            if file_vec[i][j] == 'S'
                && file_vec[i][j+2] == 'S'
                && file_vec[i+1][j+1] == 'A'
                && file_vec[i+2][j] == 'M'
                && file_vec[i+2][j+2] == 'M'
            {
                total += 1;
            }
            // sam mas check
            if file_vec[i][j] == 'S'
                && file_vec[i][j+2] == 'M'
                && file_vec[i+1][j+1] == 'A'
                && file_vec[i+2][j] == 'S'
                && file_vec[i+2][j+2] == 'M'
            {
                total += 1;
            }
        }
    }

    println!("{}", total);
}
