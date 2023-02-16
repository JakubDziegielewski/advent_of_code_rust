use std::{collections::HashSet, fs};

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Cannot read this file");
    let lines: Vec<&str> = contents.lines().collect();
    chall_one(&lines);
    chall_two(&lines);
}

fn chall_one(lines: &Vec<&str>) {
    let mut h_position: (i32, i32) = (0, 0);
    let mut t_position: (i32, i32) = (0, 0);
    let mut set_of_t_postions: HashSet<(i32, i32)> = HashSet::new();

    for line in lines {
        let parts: Vec<&str> = line.split(" ").collect();
        let moves: usize = parts[1].parse().unwrap();
        if parts[0] == "R" {
            for _ in 0..moves {
                h_position.0 += 1;
                find_square_of_distance_difference_and_move_tail(&h_position, &mut t_position);
                set_of_t_postions.insert(t_position);
            }
        } else if parts[0] == "L" {
            for _ in 0..moves {
                h_position.0 -= 1;
                find_square_of_distance_difference_and_move_tail(&h_position, &mut t_position);
                set_of_t_postions.insert(t_position);
            }
        } else if parts[0] == "U" {
            for _ in 0..moves {
                h_position.1 += 1;
                find_square_of_distance_difference_and_move_tail(&h_position, &mut t_position);
                set_of_t_postions.insert(t_position);
            }
        } else {
            // move == "D"
            for _ in 0..moves {
                h_position.1 -= 1;
                find_square_of_distance_difference_and_move_tail(&h_position, &mut t_position);
                set_of_t_postions.insert(t_position);
            }
        }
    }
    println!("{}", set_of_t_postions.len());
}

fn find_square_of_distance_difference_and_move_tail(
    h_position: &(i32, i32),
    mut t_position: &mut (i32, i32),
) {
    let x_diff: i32 = h_position.0 - t_position.0;
    let y_diff: i32 = h_position.1 - t_position.1;

    let square_of_distance_difference: i32 = x_diff.pow(2) + y_diff.pow(2);
    if square_of_distance_difference == 4 {
        if x_diff == 0 {
            t_position.1 += y_diff / 2;
        } else {
            t_position.0 += x_diff / 2;
        }
    } else if square_of_distance_difference == 5 {
        if x_diff.abs() == 1 {
            t_position.1 += y_diff / 2;
            t_position.0 += x_diff
        } else {
            t_position.0 += x_diff / 2;
            t_position.1 += y_diff;
        }
    } else if square_of_distance_difference == 8 {
        t_position.1 += y_diff / 2;
        t_position.0 += x_diff / 2;
    }
}

fn chall_two(lines: &Vec<&str>) {
    let length: usize = 10;
    let mut positions: Vec<(i32, i32)> = vec![(0, 0); length];
    let mut set_of_t_postions: HashSet<(i32, i32)> = HashSet::new();

    for line in lines {
        let parts: Vec<&str> = line.split(" ").collect();
        let moves: usize = parts[1].parse().unwrap();
        if parts[0] == "R" {
            for _ in 0..moves {
                positions[0].0 += 1;
                move_all_parts(&mut positions);
                set_of_t_postions.insert(positions[length - 1]);
            }
        } else if parts[0] == "L" {
            for _ in 0..moves {
                positions[0].0 -= 1;
                move_all_parts(&mut positions);
                set_of_t_postions.insert(positions[length - 1]);
            }
        } else if parts[0] == "U" {
            for _ in 0..moves {
                positions[0].1 += 1;
                move_all_parts(&mut positions);
                set_of_t_postions.insert(positions[length - 1]);
            }
        } else {
            // move == "D"
            for _ in 0..moves {
                positions[0].1 -= 1;
                move_all_parts(&mut positions);
                set_of_t_postions.insert(positions[length - 1]);
            }
        }
    }
    println!("{}", set_of_t_postions.len());
}

fn move_all_parts(positions: &mut Vec<(i32, i32)>) {
    for i in 1..positions.len() {
        find_square_of_distance_difference_and_move_tail(
            &positions[i - 1].clone(),
            &mut positions[i],
        );
    }
}
