use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let chall_one: u32 = chall_one();
    println!("Challange 1 result: {chall_one}");
    let chall_two: u32 = chall_two();
    println!("Challange 2 result: {chall_two}")
}

fn chall_one() -> u32 {
    let game_translator =
        HashMap::from([("A", 1), ("B", 2), ("C", 3), ("X", 1), ("Y", 2), ("Z", 3)]);
    let mut score: u32 = 0;
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("input") {
        // Consumes the iterator, returns an (Optional) String
        //let mut sum: u32 = 0;
        for line in lines {
            if let Ok(symbols) = line {
                //println!("{}", number);
                let game = symbols.split_whitespace();
                let vec: Vec<&str> = game.collect();
                if game_translator[vec[1]] == (game_translator[vec[0]] % 3 + 1) {
                    score = score + game_translator[vec[1]] + 6;
                } else if game_translator[vec[1]] == game_translator[vec[0]] {
                    score = score + game_translator[vec[1]] + 3;
                } else {
                    score = score + game_translator[vec[1]];
                }
            }
        }
    }
    score
}

fn chall_two() -> u32 {
    let game_translator =
        HashMap::from([("A", 1), ("B", 2), ("C", 3), ("X", 0), ("Y", 3), ("Z", 6)]);
    let mut score: u32 = 0;
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("input") {
        // Consumes the iterator, returns an (Optional) String
        //let mut sum: u32 = 0;
        for line in lines {
            if let Ok(symbols) = line {
                //println!("{}", number);
                let game = symbols.split_whitespace();
                let vec: Vec<&str> = game.collect();
                if game_translator[vec[1]] == 6 {
                    score = score + (game_translator[vec[0]] % 3 + 1) + 6;
                } else if game_translator[vec[1]] == 3 {
                    score = score + game_translator[vec[0]] + 3;
                } else {
                    score = score + ((game_translator[vec[0]] % 3 + 1) % 3 +1);
                }
            }
        }
    }
    score
}
// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
