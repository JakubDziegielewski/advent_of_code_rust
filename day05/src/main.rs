use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file");

    let lines: Vec<&str> = contents.split("\n").collect();
    let mut stack_slice = Vec::from(&lines[..8]);
    stack_slice.reverse();
    let command_slice = Vec::from(&lines[10..]);
    let stacks = chall_one(&stack_slice, &command_slice);
    for mut s in stacks {
        let top = s.pop().unwrap();
        print!("{top}");
    }
    println!();

    let stacks = chall_two(&stack_slice, &command_slice);
    for mut s in stacks {
        let top = s.pop().unwrap();
        print!("{top}");
    }
    println!();
}

fn chall_two(stack_slice: &Vec<&str>, command_slice: &Vec<&str>) -> Vec<Vec<char>> {
    let mut stacks: Vec<Vec<char>> = vec![
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
    ];
    
    for &line in stack_slice {
        let mut i: usize = 0;
        while i < 9 {
            if !(&line[1 + i * 4..2 + i * 4].trim().is_empty()) {
                let c: &Vec<char> = &line[1 + i * 4..2 + i * 4].chars().collect();
                stacks[i].push(c[0]);
            }
            i = i + 1;
        }
    }

    /*let iter: [usize; 9] = [0, 1, 2, 3, 4, 5, 6, 7, 8];
    for &line in stack_slice {
        for i in iter {
            if !(&line[1 + i * 4..2 + i * 4].trim().is_empty()) {
                let c: &Vec<char> = &line[1 + i * 4..2 + i * 4].chars().collect();
                stacks[i].push(c[0]);
            }
            println!("{:?}", stacks[i]);
        }
    }*/

    for &command in command_slice {
        let intsruction: Vec<&str> = command.split(" ").collect();

        //print!("{} {} {}", intsruction[1], intsruction[3], intsruction[5]);
        //println!();
        let limit: usize = intsruction[1].parse().unwrap();
        let first_stack: usize = intsruction[3].parse().unwrap();
        let second_stack: usize = intsruction[5].parse().unwrap();

        //print!("{}{}{}", limit, first_stack, second_stack);
        //println!();
        let mut iterator: usize = 0;
        let mut temp: Vec<char> = Vec::new();
        while iterator < limit {
            temp.push(stacks[first_stack - 1].pop().unwrap());
            iterator = iterator + 1;
        }
        iterator = 0;
        while iterator < limit {
            stacks[second_stack - 1].push(temp.pop().unwrap());
            iterator = iterator + 1;
        }
    }

    stacks
}

fn chall_one(stack_slice: &Vec<&str>, command_slice: &Vec<&str> ) -> Vec<Vec<char>>{
    let mut stacks: Vec<Vec<char>> =vec![Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new()];
    let iter: [usize; 9] = [0 ,1, 2, 3, 4, 5, 6, 7, 8];
    for &line in stack_slice{
        for i in iter{

            if !(&line[1+i*4..2+i*4].trim().is_empty()){
                let c: &Vec<char> = &line[1+i*4..2+i*4].chars().collect();
                stacks[i].push(c[0]);

            }
        }
    }
    for &command in command_slice {

        let intsruction: Vec<&str> = command.split(" ").collect();

        //print!("{} {} {}", intsruction[1], intsruction[3], intsruction[5]);
        //println!();
        let limit: usize = intsruction[1].parse().unwrap();
        let first_stack: usize = intsruction[3].parse().unwrap();
        let second_stack: usize = intsruction[5].parse().unwrap();

        //print!("{}{}{}", limit, first_stack, second_stack);
        //println!();
        let mut iterator: usize = 0;
         while iterator < limit {
            let temp = stacks[first_stack - 1].pop().unwrap();
            stacks[second_stack - 1].push(temp);
            iterator = iterator + 1;
         }

    }


    stacks
}
 
