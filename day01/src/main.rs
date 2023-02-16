use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // File hosts must exist in current path before this produces output
    chall_one();
   chall_two();
}

fn chall_one(){
    if let Ok(lines) = read_lines("input") {
        // Consumes the iterator, returns an (Optional) String
        let mut sum: u32 = 0;
        let mut first: u32 = 0;
        for line in lines {
            if let Ok(number) = line {
                if number.len() == 0 {
                    if sum > first{
                      first = sum
                    }
                    sum = 0;
                }
                else {
                    let num: u32 = number.parse().unwrap();
                    sum = sum + num
                }
                //println!("{}", number);
            }
        }
        println!("result: {first}")
    }
}
fn chall_two(){
    if let Ok(lines) = read_lines("input") {
        // Consumes the iterator, returns an (Optional) String
        let mut sum: u32 = 0;
        let mut first: u32 = 0;
        let mut second: u32 = 0;
        let mut third: u32 = 0;
        for line in lines {
            if let Ok(number) = line {
                if number.len() == 0 {
                    if sum > third{
                        if sum > second{
                            if sum > first{
                                third = second;
                                second = first;
                                first = sum;
                            }
                            else {
                                third = second;
                                second = sum;
                            }
                        }
                        else {
                            third = sum
                        }
                    }
                    sum = 0;
                }
                else {
                    let num: u32 = number.parse().unwrap();
                    sum = sum + num
                }
                //println!("{}", number);
            }
        }
        let res = first + second + third;
        println!("result: {res}")
    }
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
