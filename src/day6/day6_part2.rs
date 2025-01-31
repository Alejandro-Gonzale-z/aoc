use std::fs::read_to_string;

fn main() {
    let file_content: String = read_to_string("./src/day6.txt").unwrap();
    // let test_content: String = read_to_string("./src/day6_test_input.txt").unwrap();

    let mut original_map: Vec<Vec<char>> = file_content
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let mut map = original_map.clone();

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
    println!("{}", visited_locations.len());

    // Part Two Solution
    // reset the map after altering it in part one above
    let mut loop_counter = 0;
    
    map = original_map.clone();
    
    for location in visited_locations {
        let mut visited_locations_directions: Vec<(usize, usize, char)> = Vec::new();
        let mut current_index_direction: (usize, usize, char) =
            find_guard_position_direction(&map).unwrap();

        map[location.0][location.1] = '#';

        while current_index_direction.0 > 0
            && current_index_direction.1 > 0
            && current_index_direction.0 < map.len() - 1
            && current_index_direction.1 < map[0].len() - 1
        {
            // if the location to be replaced is the location of the guard skip
            if location.0 == 6 && location.1 == 4 {
                break;
            }
            move_guard(
                &mut map,
                current_index_direction.0,
                current_index_direction.1,
            );

            current_index_direction = find_guard_position_direction(&map).unwrap();
            if visited_locations_directions.contains(&current_index_direction) {
                loop_counter += 1;
                visited_locations_directions.clear();
                println!("found a loop");
                break;
            } else {
                visited_locations_directions.push(current_index_direction);
            }
        }
        // reset map after each run
        map = original_map.clone();
    }
    println!("Loops: {}", loop_counter);
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
fn find_guard_position_direction(map: &Vec<Vec<char>>) -> Option<(usize, usize, char)> {
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            match map[i][j] {
                '^' | 'v' | '>' | '<' => return Some((i, j, map[i][j])),
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
            println!("cant find guard");
        }
    }
}
