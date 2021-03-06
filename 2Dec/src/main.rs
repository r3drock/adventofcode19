use std::fs;
use std::process;

fn read_data() -> Vec<usize> {
    let mut program: Vec<usize> = Vec::new();
    let data = fs::read_to_string("data")
        .expect("Something went wrong reading the file");
    for line in data.split(',') {
        match line.parse::<usize>() {
            Ok(x) => program.push(x),
            Err(_) => (),
        };
    }
    program
}

fn run_program(mut program: Vec<usize>, noun: usize, verb: usize, print: bool) -> usize {
    program[1] = noun;
    program[2] = verb;
    if print {print_program(&program);}

    let size = program.len();
    let mut index = 0;
    while index < size - (size % 4) {
        let index_to_overwrite = program[index+3];
        program[index_to_overwrite] = match program[index] {
            1 => program[program[index+1]] + program[program[index+2]],
            2 => program[program[index+1]] * program[program[index+2]],
            99 => break,
            _ => panic!("illegal opcode"),
        };
        index += 4;
    }
    if print {print_program(&program);}
    program[0]
}

fn print_program(program: &Vec<usize>) {
    let mut index_in_line = 0;
    println!("------------------------------------------------");
    for element in program {
        print!("{:>10}, ", element);
        if index_in_line == 3 {
            println!();
            index_in_line = 0; 
        } else {
            index_in_line += 1;
        };
    }
    println!("\n------------------------------------------------");
}

fn part1() {
    let program: Vec<usize> = read_data();
    println!("Result: {}", run_program(program, 12, 2, true));
}

fn part2() {
    let program: Vec<usize> = read_data();
    for noun in 1..100 {
        for verb in 1..100 {
            let programcopy = program.clone();
            if 19690720 == run_program(programcopy, noun, verb, false) {
                println!("noun: {}, verb: {}", noun, verb);
                std::process::exit(0);
            }
        }
    }
}
fn main() {
    part1();
    part2();
}
