use std::{collections::HashMap, fs};

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let lines: Vec<&str> = contents.lines().collect();
    challenges(lines);
}

fn challenges(lines: Vec<&str>) {
    let mut current_dir = String::new();
    let scores: &mut HashMap<String, u64> = &mut HashMap::new();
    let mut score: u64;
    let mut res: u64 = 0;

    for line in &lines[..] {
        let words: Vec<&str> = line.split(" ").collect();
        if words[1] == "cd" {
            //println!("{score}");
            current_dir = change_dir(current_dir, words[2]);
        } else if words[1] == "ls" {
        } else if words[0] != "dir" {
            let parts: Vec<&str> = current_dir.split("?").collect();
            score = words[0].parse::<u64>().unwrap();
            for i in 2..=parts.len()
            {
                let k = parts[..i].join("?");
                let v = scores.entry(k).or_insert(0);
                *v += score;
            }
            
        }
    }

    let space_needed: u64 = 30_000_000;
    let free_space: u64 = 70_000_000 - *scores.get("?/").unwrap();
    let min_size = space_needed - free_space;
    let mut res2: u64 = 30_000_000; 
    for k in scores.keys() {
        if *scores.get(k).unwrap() < 100_000 {
            res += *scores.get(k).unwrap();
        }
        if *scores.get(k).unwrap() >= min_size && *scores.get(k).unwrap() <= res2{
            res2 = *scores.get(k).unwrap();
        }

    }
    println!("{res}");
    println!("{res2}");

}


fn change_dir(mut current_dir: String, dest: &str) -> String {
    let mut parts: Vec<&str> = current_dir.split("?").collect();
    if dest == ".." {
        parts.pop();
        current_dir = String::from(parts.join("?"));
    } else {
        parts.push(dest);
        current_dir = String::from(parts.join("?"));
    }

    current_dir
}
