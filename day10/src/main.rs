use std::fs;
fn main() {
    let contents = fs::read_to_string("input.txt").expect("No suchfile");

    let lines: Vec<&str> = contents.lines().collect();
    chall_one(&lines);
    chall_two(&lines);
}

fn chall_one(lines: &Vec<&str>) {
    let mut next_circle: i32 = 1;
    let mut register: i32 = 1;
    let mut power: i32;
    let mut sum: i32 = 0;

    for line in lines {
        let parts: Vec<&str> = line.split(" ").collect();
        if parts[0] == "noop" {
            next_circle += 1;
        } else {
            next_circle += 1;
            if next_circle % 40 == 20 {
                power = next_circle * register;
                sum += power;
            }
            next_circle += 1;
            register += parts[1].parse::<i32>().unwrap();
        }
        if next_circle % 40 == 20 {
            power = next_circle * register;
            sum += power;
        }
    }
    println!("{sum}");
}

fn chall_two(lines: &Vec<&str>) {
    let mut next_circle: i32 = 1;
    let mut register: i32 = 1;
    for line in lines {
        let parts: Vec<&str> = line.split(" ").collect();
        if next_circle == 41 {
            next_circle = 1;
            println!("");
        }
        draw_symbol(next_circle, register);
        if parts[0] == "noop" {
            next_circle += 1;
        } else {
            next_circle += 1;
            if next_circle == 41 {
                next_circle = 1;
                println!("");
            }
            draw_symbol(next_circle, register);
            next_circle += 1;
            register += parts[1].parse::<i32>().unwrap();
        }
    }
}
fn draw_symbol(next_circle: i32, register: i32) {
    if next_circle - 1 <= register + 1 && next_circle - 1 >= register - 1 {
        print!("#");
    } else {
        print!(".");
    }
}
