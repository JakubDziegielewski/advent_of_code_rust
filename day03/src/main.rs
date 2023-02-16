use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::fs::File;
use std::io::{self, BufRead};
use std::iter::FromIterator;
use std::path::Path;

fn main() {
    let res_one = chall_one();
    let res_two = chall_two();
    println!("Result one: {}", res_one);
    println!("Result two: {}", res_two);
}

fn chall_one() -> u32 {
    let priorities: HashMap<char, u32> = create_priorities();
    let mut sum: u32 = 0;
    if let Ok(lines) = read_lines("input") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(racksack) = line {
                let compartment_size: usize = racksack.len() / 2;
                let first_compertment = &racksack[..compartment_size];
                let second_compartment = &racksack[compartment_size..];

                let first_vec: Vec<char> = first_compertment.chars().collect();
                let second_vec: Vec<char> = second_compartment.chars().collect();

                let first_set = vec_to_set(first_vec);
                let second_set = vec_to_set(second_vec);

                let inter: HashSet<_> = first_set.intersection(&second_set).collect();
                let mut result: &&char = &&'*';
                for letter in inter.iter() {
                    result = letter;
                }
                sum = sum + priorities[result]
            }
        }
    }
    sum
}

fn chall_two() -> u32 {
    let priorities: HashMap<char, u32> = create_priorities();
    let mut sum: u32 = 0;
    let contents = fs::read_to_string("input").expect("Should have been able to read the file");

    let lines: Vec<&str> = contents.split("\n").collect();

    let limit = lines.len() / 3;
    for i in 0..limit {
        let iterator = i * 3;
        let first = lines[iterator];
        let second = lines[iterator + 1];
        let third = lines[iterator + 2];

        let first_vec: Vec<char> = first.chars().collect();
        let second_vec: Vec<char> = second.chars().collect();
        let third_vec: Vec<char> = third.chars().collect();

        let first_set = vec_to_set(first_vec);
        let second_set = vec_to_set(second_vec);
        let third_set: HashSet<char> = vec_to_set(third_vec);

        let inter: HashSet<_> = first_set.intersection(&second_set).collect();
        let mut inter_as_set: HashSet<char> = HashSet::new();
        for letter in inter.iter() {
            inter_as_set.insert(**letter);
        }

        let final_inter: HashSet<_> = third_set.intersection(&inter_as_set).collect();
        let mut result: &&char = &&'*';
        for letter in final_inter.iter() {
            result = letter;
        }
        sum = sum + priorities[result];
    }
    sum
}

fn create_priorities() -> HashMap<char, u32> {
    let mut priorities: HashMap<char, u32> = HashMap::new();
    let mut priority: u32 = 1;
    for letter in 'a'..='z' {
        priorities.insert(letter, priority);
        priority = priority + 1;
    }
    for letter in 'A'..='Z' {
        priorities.insert(letter, priority);
        priority = priority + 1;
    }
    priorities
}
fn vec_to_set(vec: Vec<char>) -> HashSet<char> {
    HashSet::from_iter(vec)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
