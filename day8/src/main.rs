use std::cmp::max;
use std::ops::Range;
use std::{collections::HashSet, fs, usize};
fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let lines: Vec<&str> = contents.lines().collect();
    chall_one(lines.clone());
    chall_two(lines);
}
#[derive(PartialEq, Eq, Hash, Debug)]
struct Tree {
    x: usize,
    y: usize,
}

fn chall_one(lines: Vec<&str>) {
    let mut visible_trees: HashSet<Tree> = HashSet::new();
    let mut tallest_trees_in_collumns: Vec<i8> = vec![-1; lines[0].len()];
    let length = lines[0].len();

    iterate_over_trees(
        0..lines.len(),
        &lines,
        length,
        &mut tallest_trees_in_collumns,
        &mut visible_trees,
    );

    let mut tallest_trees_in_collumns: Vec<i8> = vec![-1; lines[0].len()];

    iterate_over_trees_reverse(
        0..lines.len(),
        &lines,
        length,
        &mut tallest_trees_in_collumns,
        &mut visible_trees,
    );
    println!("{:?}", visible_trees.len());
}
fn insert_if_visible(
    height: i8,
    tallest_trees_in_collumns: &mut Vec<i8>,
    tallest_tree_in_row: &mut i8,
    visible_trees: &mut HashSet<Tree>,
    i: usize,
    j: usize,
) {
    if height > tallest_trees_in_collumns[j] {
        tallest_trees_in_collumns[j] = height;
        let tree: Tree = Tree { x: j, y: i };
        visible_trees.insert(tree);
    }
    if height > *tallest_tree_in_row {
        *tallest_tree_in_row = height;
        let tree: Tree = Tree { x: j, y: i };
        visible_trees.insert(tree);
    }
}

fn iterate_over_trees(
    range: Range<usize>,
    lines: &Vec<&str>,
    length: usize,
    mut tallest_trees_in_collumns: &mut Vec<i8>,
    mut visible_trees: &mut HashSet<Tree>,
) {
    for i in range {
        let mut tallest_tree_in_row: i8 = -1;
        let trees: Vec<char> = lines[i].chars().collect();
        for j in 0..length {
            let height: i8 = trees[j].to_string().parse().unwrap();
            insert_if_visible(
                height,
                &mut tallest_trees_in_collumns,
                &mut tallest_tree_in_row,
                &mut visible_trees,
                i,
                j,
            )
        }
    }
}

fn iterate_over_trees_reverse(
    range: Range<usize>,
    lines: &Vec<&str>,
    length: usize,
    mut tallest_trees_in_collumns: &mut Vec<i8>,
    mut visible_trees: &mut HashSet<Tree>,
) {
    for i in range.rev() {
        let mut tallest_tree_in_row: i8 = -1;
        let trees: Vec<char> = lines[i].chars().collect();
        for j in (0..length).rev() {
            let height: i8 = trees[j].to_string().parse().unwrap();
            insert_if_visible(
                height,
                &mut tallest_trees_in_collumns,
                &mut tallest_tree_in_row,
                &mut visible_trees,
                i,
                j,
            )
        }
    }
}

fn chall_two(lines: Vec<&str>) {
    let mut map: Vec<Vec<i8>> = Vec::new();
    let length = lines[0].len();
    for i in 0..lines.len() {
        let mut line: Vec<i8> = Vec::new();
        let trees: Vec<char> = lines[i].chars().collect();
        for j in 0..length {
            let height: i8 = trees[j].to_string().parse().unwrap();
            line.push(height);
        }
        map.push(line);
    }

    let mut result: i32 = 0;

    for i in 0..lines.len() {
        let trees: Vec<char> = lines[i].chars().collect();
        for j in 0..length {
            let mut sights = (0, 0, 0, 0);
            let height: i8 = trees[j].to_string().parse().unwrap();
            while i - sights.0 > 0 {
                sights.0 += 1;
                if height <= map[i - sights.0][j] {
                    break;
                }
            }
            while i + sights.1 < lines.len() - 1 {
                sights.1 += 1;
                if height <= map[i + sights.1][j] {
                    break;
                }
            }
            while j - sights.2 > 0 {
                sights.2 += 1;
                if height <= map[i][j - sights.2] {
                    break;
                }
            }
            while j + sights.3 < lines.len() - 1 {
                sights.3 += 1;
                if height <= map[i][j + sights.3] {
                    break;
                }
            }
            let score: i32 = (sights.0 * sights.1 * sights.2 * sights.3) as i32;
            result = max(result, score);
            
            
        }
    }

    println!("{}", result);
}
