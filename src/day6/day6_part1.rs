use std::fs::read_to_string;

fn main() {
    let file_content: String = read_to_string("./src/day6.txt").unwrap();
    //let test_content: String = read_to_string("./src/day6_test_input.txt").unwrap();

    let mut map: Vec<Vec<char>> = file_content
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut visited_locations: Vec<(usize, usize)> = Vec::new();
    let mut current_index: (usize, usize) = find_guard_position(&map).unwrap();
    
    while current_index.0 != map.len() - 1 && current_index.1 != map[0].len() - 1 {
        // println!("index: {:?} guard: {}", current_index, map[current_index.0][current_index.1]);
        move_guard(&mut map, current_index.0, current_index.1);

        current_index = find_guard_position(&map).unwrap();
        if !visited_locations.contains(&current_index) {
            visited_locations.push(current_index);
        }
    }

}

fn find_guard_position(map: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            match map[i][j] {
                '^' | 'v' | '>' | '<' => return Some((i, j)),
                _ => continue,
            }
        }
    }
    None
}

// need bounds checking?
fn move_guard(map: &mut Vec<Vec<char>>, x: usize, y: usize) {
    let guard = map[x][y];
    match guard {
        '^' => {
            if map[x - 1][y] == '#' {
                map[x][y] = '>';
            } else {
                map[x - 1][y] = '^';
                map[x][y] = '.';
            }
        }
        'v' => {
            if map[x + 1][y] == '#' {
                map[x][y] = '<';
            } else {
                map[x + 1][y] = 'v';
                map[x][y] = '.';
            }
        }
        '>' => {
            if map[x][y + 1] == '#' {
                map[x][y] = 'v';
            } else {
                map[x][y + 1] = '>';
                map[x][y] = '.';
            }
        }
        '<' => {
            if map[x][y - 1] == '#' {
                map[x][y] = '^';
            } else {
                map[x][y - 1] = '<';
                map[x][y] = '.';
            }
        }
        _ => {
            println!("placeholder");
        }
    }
}
