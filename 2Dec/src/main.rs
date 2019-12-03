use std::fs;

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
    program[1] = 12;
    program[2] = 2;
    program
}

fn run_program(mut program: Vec<usize>) -> usize {
    print_program(&program);

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
    print_program(&program);
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

fn main() {
    let program: Vec<usize> = read_data();
    println!("Result: {}", run_program(program));
}
