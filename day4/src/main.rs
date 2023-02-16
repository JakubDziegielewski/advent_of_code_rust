use std::{fs, process::exit};

fn main() {
    let contents = fs::read_to_string("input").expect("Should have been able to read the file");

    let lines: Vec<&str> = contents.split("\n").collect();
    let result_chall_one = chall_one(&lines);
    let result_chall_two = chall_two(&lines);
    println!("Chall one: {result_chall_one}");
    println!("Chall two: {result_chall_two}");
}
fn chall_two(lines: &Vec<&str>) -> u32 {
    let mut overlaping_sections: u32 = 0;
    for line in lines {
        let (first_elf, second_elf) = prepare_sections(line);
        if sections_overlap(first_elf, second_elf){
            overlaping_sections = overlaping_sections + 1;
        }
    }
    overlaping_sections
}
fn chall_one(lines: &Vec<&str>) -> u32 {
    let mut overlaping_sections: u32 = 0;
    for line in lines {
        let (first_elf, second_elf) = prepare_sections(line);
        if first_section_is_bigger(first_elf, second_elf)
            && sections_overlap_fully(first_elf, second_elf)
        {
            overlaping_sections = overlaping_sections + 1;
        } else if sections_overlap_fully(second_elf, first_elf) {
            overlaping_sections = overlaping_sections + 1;
        }
    }
    overlaping_sections
}

fn prepare_sections(line: &str) -> ((u32, u32), (u32, u32)) {
    let sections = split_str(line);
    let first_elf = find_sections(sections.0);
    let second_elf = find_sections(sections.1);
    (first_elf, second_elf)
}

fn first_section_is_bigger(first_elf: (u32, u32), second_elf: (u32, u32)) -> bool {
    let first_assigment = first_elf.1 - first_elf.0;
    let second_assignment = second_elf.1 - second_elf.0;
    first_assigment > second_assignment
}
//fn chall_two()
fn sections_overlap(first_assignment: (u32, u32), second_assignment: (u32, u32)) -> bool {
    if first_assignment.0 > second_assignment.1 {
        return false;
    }
    if second_assignment.0 > first_assignment.1{
        return false;
    }
    true
}

fn sections_overlap_fully(bigger_assignment: (u32, u32), smaller_assignment: (u32, u32)) -> bool {
    if bigger_assignment.0 > smaller_assignment.0 {
        return false;
    }
    if bigger_assignment.1 < smaller_assignment.1 {
        return false;
    }
    true
}
fn split_str(line: &str) -> (&str, &str) {
    let bytes = line.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b',' {
            return (&line[..i], &line[i + 1..]);
        }
    }
    ("", "")
}
fn find_sections(s: &str) -> (u32, u32) {
    let bytes = s.as_bytes();
    let mut first_section: u32 = 0;
    let mut last_section: u32 = 0;
    for (i, &item) in bytes.iter().enumerate() {
        if item == b'-' {
            first_section = match s[..i].parse() {
                Ok(first) => first,
                Err(_) => exit(1),
            };
            last_section = match s[i + 1..].parse() {
                Ok(second) => second,
                Err(_) => exit(2),
            };
        }
    }
    (first_section, last_section)
}
