use std::{collections::{VecDeque, HashMap}, fs};
fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let res_one = chall(&contents, 4);
    let res_two = chall(&contents, 14);

    println!("{res_one}");
    println!("{res_two}");

}

fn chall(input: &str, cap: usize) -> u32 {
    let letters: Vec<char>  = input.chars().collect();
    let mut deque: VecDeque<char> = VecDeque::with_capacity(cap);
    let mut status: HashMap<char, u8> = HashMap::new();
    for i in &letters[..cap]{
        deque.push_back(*i);
        let s = status.entry(*i).or_insert(0);
        *s = *s + 1;
    }
    
    for (i, c) in letters[cap..].iter().enumerate(){
        if *status.values().max().unwrap() == 1 as u8 {
            return i as u32 + cap as u32;
        }
        let removed = deque.pop_front().unwrap();
        let val = status.entry(removed).or_insert(1);
        *val = *val - 1;
        if *val == 0{
        status.remove_entry(&removed);
        }
        deque.push_back(*c);
        let s = status.entry(*c).or_insert(0);
        *s = *s + 1;
    }

    0
}

