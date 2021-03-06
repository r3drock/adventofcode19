use std::io;
use std::fs;
use std::convert::TryFrom;

fn read_data() -> Vec<isize> {
    let mut program: Vec<isize> = Vec::new();
    let data = fs::read_to_string("data")
        .expect("Something went wrong reading the file");
    for line in data.split(',') {
        match line.parse::<isize>() {
            Ok(x) => program.push(x),
            Err(_) => (),
        };
    }
    program
}

fn conv(x: isize) -> usize {
    usize::try_from(x).expect("negative instruction pointer")
}

fn get_opcode (mut value: usize) -> usize {
    let opcode = value % 10;
    value /= 10;
    opcode + (value % 10) * 10
}

fn get_modes (mut value: usize) -> (usize, usize, usize) {
    value /= 100;
    match value {
          0 => (0,0,0),
          1 => (1,0,0),
         10 => (0,1,0),
         11 => (1,1,0),
        100 => (0,0,1),
        101 => (1,0,1),
        110 => (0,1,1),
        111 => (1,1,1),
        _ => panic!("unsupported parameter mode"),
    }
}

fn access(mode: usize, index: usize, program: &Vec<isize>) -> isize {
    if mode == 0 {program[conv(program[index])]} else {program[index]}
}

fn run_program(mut program: Vec<isize>) {
    let size = program.len();
    let mut ip: usize = 0;
    while ip < size - (size % 4) {
        let (mode1, mode2, mode3) = get_modes(conv(program[ip]));

        match get_opcode(conv(program[ip])) {
            //add
            1 => {
                let index_to_overwrite = if mode3 == 0 {conv(program[ip+3])} else {ip+3};
                program[index_to_overwrite] = access(mode1, ip+1, &program) +
                                              access(mode2, ip+2, &program);
                ip += 4;
            }

            //mul
            2 => {
                let index_to_overwrite = if mode3 == 0 {conv(program[ip+3])} else {ip+3};
                program[index_to_overwrite] = access(mode1, ip+1, &program) *
                                              access(mode2, ip+2, &program);
                ip += 4;
            }

            //input
            3 => {
                let mut input = String::new();
                println!("Please input a number.");
                io::stdin().read_line(&mut input)
                    .expect("error reading line");

                let input : isize = match input.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };

                let index_to_overwrite = if mode1 == 0 {conv(program[ip+1])} else {ip+1};
                program[index_to_overwrite] = input;
                ip += 2;
            },

            //output
            4 => {
                println!("{}", conv(access(mode1, ip+1, &program)));
                ip += 2;
            },

            //jump-if-true
            5 => {
                ip = if access(mode1, ip+1, &program) != 0 {
                    conv(access(mode2, ip+2, &program))
                } else {
                    ip + 3
                };
            },

            //jump-if-false
            6 => {
                ip = if access(mode1, ip+1, &program) == 0 {
                    conv(access(mode2, ip+2, &program))
                } else {
                    ip + 3
                };
            },

            //less than
            7 => {
                let index_to_overwrite = if mode3 == 0 {conv(program[ip+3])} else {ip+3};
                program[index_to_overwrite] =
                    if access(mode1, ip+1, &program) < access(mode2, ip+2, &program) {
                    1
                } else {
                    0
                };
                ip += 4;
            }

            //equals
            8 => {
                let index_to_overwrite = if mode3 == 0 {conv(program[ip+3])} else {ip+3};
                program[index_to_overwrite] =
                    if access(mode1, ip+1, &program) == access(mode2, ip+2, &program) {
                    1
                } else {
                    0
                };
                ip += 4;
            }
            99 => break,
            _ => panic!("illegal opcode"),
        };
        
    }
}

fn print_program(program: &Vec<isize>) {
    println!("------------------------------------------------");
    let len = program.len();
    let mut i = 0;
    while i < len {
        let numbers_remaining_in_line =
            match get_opcode(conv(program[i])){
                1  => 4,
                2  => 4,
                3  => 2,
                4  => 2,
                5  => 3,
                6  => 3,
                7  => 4,
                8  => 4,
                99 => 1,
                _  => 1,
            };
        if numbers_remaining_in_line - 1 + i >= len {break;};
        for j in 0..numbers_remaining_in_line {
            print!("{:>10}, ", program[i + j]);
        }
        println!();
        i += numbers_remaining_in_line;
    }
    println!("------------------------------------------------");
}

fn main() {
    let program = read_data();
    run_program(program);
}
