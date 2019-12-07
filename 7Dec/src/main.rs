use std::io;
use std::fs;
use std::convert::TryFrom;
use permutohedron::Heap;

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

fn run_program(mut program: Vec<isize>, input_vec: Vec<isize>, terminated: &mut bool) -> Option<isize> {
    let mut output: Option<isize> = None;
    let mut input_vec_iterator = input_vec.iter();
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
            //reads from input_vec and if it is not present from stdin
            3 => {
                let input: isize = match input_vec_iterator.next() {
                    Some(num) => *num,
                    None => {
                        let mut input_string = String::new();
                        println!("Please input a number.");
                        io::stdin().read_line(&mut input_string)
                            .expect("error reading line");

                        match input_string.trim().parse() {
                            Ok(num) => num,
                            Err(_) => continue,
                        }
                    },
                };

                let index_to_overwrite = if mode1 == 0 {conv(program[ip+1])} else {ip+1};
                program[index_to_overwrite] = input;
                ip += 2;
            },

            //output
            4 => {
                output = Some(access(mode1, ip+1, &program));
                //println!("{}", output.unwrap());
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
            99 => break,//{*terminated = true; break;},
            _ => panic!("illegal opcode"),
        };

    }
    output
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

fn part1() {
    let program = read_data();

    let mut data = vec![0, 1, 2, 3, 4];
    let heap = Heap::new(&mut data);

    let mut permutations = Vec::new();
    for data in heap {
        permutations.push(data.clone());
    }

    let mut terminated = false;
    let mut max = 0;
    for input in permutations {
        let mut output = run_program(program.clone(), vec![input[0],0], &mut terminated).unwrap();
        output = run_program(program.clone(), vec![input[1], output], &mut terminated).unwrap();
        output = run_program(program.clone(), vec![input[2], output], &mut terminated).unwrap();
        output = run_program(program.clone(), vec![input[3], output], &mut terminated).unwrap();
        output = run_program(program.clone(), vec![input[4], output], &mut terminated).unwrap();
        if output > max {max = output;};
    }
    println!("Highest signal that can be sent to the thrusters: {}", max);
}

fn part2() {
    let program = read_data();

    let mut data = vec![5, 6, 7, 8, 9];
    let heap = Heap::new(&mut data);

    let mut permutations = Vec::new();
    for data in heap {
        permutations.push(data.clone());
    }

    let mut terminated = false;
    let mut max = 0;
    for input in permutations {
            let program_A = program.clone();
            let program_B = program.clone();
            let program_C = program.clone();
            let program_D = program.clone();
            let program_E = program.clone();
            let mut output = run_program(program.clone(), vec![input[0], 0], &mut terminated).unwrap();
            output = run_program(program.clone(), vec![input[1], output], &mut terminated).unwrap();
            let output = run_program(program.clone(), vec![input[2], output], &mut terminated).unwrap();
            let mut output = run_program(program.clone(), vec![input[3], output], &mut terminated).unwrap();
            let last_amplifier_output = run_program(program.clone(), vec![input[4], output], &mut terminated).unwrap();
            if terminated {
                if last_amplifier_output > max {max = last_amplifier_output;};
                continue;
            }
            loop {
                output = run_program(program.clone(), vec![output], &mut terminated).unwrap();
                if terminated {break;}
                output = run_program(program.clone(), vec![output], &mut terminated).unwrap();
                if terminated {break;}
                output = run_program(program.clone(), vec![output], &mut terminated).unwrap();
                if terminated {break;}
                output = run_program(program.clone(), vec![output], &mut terminated).unwrap();
                if terminated {break;}
                let last_amplifier_output = run_program(program.clone(), vec![output], &mut terminated).unwrap();
                output = last_amplifier_output;
                if terminated {break;}

            }
            if last_amplifier_output > max {max = last_amplifier_output;};
        }
    println!("Highest signal that can be sent to the thrusters: {}", max);
}

fn main() {
    part2();
}
