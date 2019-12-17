use std::io;
use std::fs;
use std::convert::TryFrom;
use permutohedron::Heap;
use std::collections::VecDeque;

#[derive(Debug)]
struct Amplifier {
    ip: usize,
    inputbuffer: VecDeque<isize>,
    program: Vec<isize>,
}


fn read_data(file_name: &str) -> Vec<isize> {
    let mut program: Vec<isize> = Vec::new();
    let data = fs::read_to_string(file_name)
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

impl Amplifier {
    fn new(program: Vec<isize>, input: Vec<isize>) -> Amplifier {
        Amplifier { inputbuffer: VecDeque::from(input), program: program, ip : 0 }
    }

    fn push_input(&mut self, input: isize) {
        self.inputbuffer.push_back(input);
    }

    fn pp(&self) {
        print!("[");
        for (i, item) in self.program.iter().enumerate() {
            if i == self.ip {
            print!(">{}<, ", item);
            } else {
            print!("{}, ", item);
            }
        }
        println!("]");
    }

    fn run_program(&mut self, debug: bool) -> Option<isize> {
        let mut output: Option<isize> = None;
        let size = self.program.len();
        while self.ip < size {
            let (mode1, mode2, mode3) = get_modes(conv(self.program[self.ip]));

            if debug {self.pp()};
            match get_opcode(conv(self.program[self.ip])) {
                //add
                1 => {
                    let index_to_overwrite = if mode3 == 0 {conv(self.program[self.ip+3])} else {self.ip+3};
                    if debug {
                        println!("ADD {}, {}, {}", access(mode1, self.ip+1, &self.program), 
                                                   access(mode2, self.ip+2, &self.program),
                                                   self.program[self.ip+3] );
                    };
                    self.program[index_to_overwrite] = access(mode1, self.ip+1, &self.program) +
                                                  access(mode2, self.ip+2, &self.program);
                    self.ip += 4;
                }

                //mul
                2 => {
                    let index_to_overwrite = if mode3 == 0 {conv(self.program[self.ip+3])} else {self.ip+3};
                    if debug {
                        println!("MUL {}, {}, {}", access(mode1, self.ip+1, &self.program), 
                                                   access(mode2, self.ip+2, &self.program),
                                                   self.program[self.ip+3] );
                    };
                    self.program[index_to_overwrite] = access(mode1, self.ip+1, &self.program) *
                                                  access(mode2, self.ip+2, &self.program);
                    self.ip += 4;
                }

                //input
                //reads from inputbuffer and if that is empty from stdin
                3 => {
                    let input: isize = match self.inputbuffer.pop_front() {
                        Some(num) => { 
                            if debug {println!("input {}", num);};
                            num
                        },
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

                    let index_to_overwrite = if mode1 == 0 {conv(self.program[self.ip+1])} else {self.ip+1};
                    self.program[index_to_overwrite] = input;
                    self.ip += 2;
                },

                //output
                4 => {
                    output = Some(access(mode1, self.ip+1, &self.program));
                    if debug { println!("outputaddr {} = {}", self.ip+1, output.unwrap()); };
                    self.ip += 2;
                    break;
                },

                //jump-if-true
                5 => {
                    self.ip = if access(mode1, self.ip+1, &self.program) != 0 {
                        if debug {
                            println!("jump {}", access(mode2, self.ip+2, &self.program));
                        };
                        conv(access(mode2, self.ip+2, &self.program))
                    } else {
                        if debug {
                            println!("jump {}", self.ip + 3);
                        };
                        self.ip + 3
                    };
                },

                //jump-if-false
                6 => {
                    self.ip = if access(mode1, self.ip+1, &self.program) == 0 {
                        if debug {
                            println!("jump {}", access(mode2, self.ip+2, &self.program));
                        };
                        conv(access(mode2, self.ip+2, &self.program))
                    } else {
                        if debug {
                            println!("jump {}", self.ip + 3);
                        };
                        self.ip + 3
                    };
                },

                //less than
                7 => {
                    let index_to_overwrite = if mode3 == 0 {conv(self.program[self.ip+3])} else {self.ip+3};
                    self.program[index_to_overwrite] =
                        if access(mode1, self.ip+1, &self.program) < access(mode2, self.ip+2, &self.program) {
                        if debug {
                                println!("lt {} {} {}", access(mode1, self.ip+1, &self.program),
                                    access(mode1, self.ip+1, &self.program), index_to_overwrite);
                            };
                        1
                    } else {
                        if debug {
                                println!("not lt {} {} {}", access(mode1, self.ip+1, &self.program),
                                    access(mode1, self.ip+1, &self.program), index_to_overwrite);
                            };
                        0
                    };
                    self.ip += 4;
                }

                //equals
                8 => {
                    let index_to_overwrite = if mode3 == 0 {conv(self.program[self.ip+3])} else {self.ip+3};
                    self.program[index_to_overwrite] =
                        if access(mode1, self.ip+1, &self.program) == access(mode2, self.ip+2, &self.program) {
                            if debug {
                                println!("equals {} {} {}", access(mode1, self.ip+1, &self.program),
                                    access(mode1, self.ip+1, &self.program), index_to_overwrite);
                            };
                            1
                        } else {
                            if debug {
                                println!("not equals {} {} {}", access(mode1, self.ip+1, &self.program),
                                    access(mode1, self.ip+1, &self.program), index_to_overwrite);
                            };
                            0
                        };
                    self.ip += 4;
                }
                99 => { if debug {
                            println!("END");
                        }
                        break
                      },
                a => panic!("illegal opcode {}", a),
            };

        }
        output
    }
}

fn part1() {
    let program = read_data("data");

    let mut data = vec![4, 3, 2, 1, 0];
    let heap = Heap::new(&mut data);

    let mut permutations = Vec::new();
    for data in heap {
        permutations.push(data.clone());
    }

    let mut max = 0;
    let debug =  false;
    for input in permutations {
        let mut amplifier_a = Amplifier::new(program.clone(), vec![input[0],0]); 
        let mut amplifier_b = Amplifier::new(program.clone(), vec![input[1]]); 
        let mut amplifier_c = Amplifier::new(program.clone(), vec![input[2]]); 
        let mut amplifier_d = Amplifier::new(program.clone(), vec![input[3]]); 
        let mut amplifier_e = Amplifier::new(program.clone(), vec![input[4]]); 

        if debug {println!("amplifier A starting"); }
        let output = amplifier_a.run_program(debug).unwrap();

        amplifier_b.push_input(output);
        if debug { println!("amplifier B starting"); }
        let output = amplifier_b.run_program(debug).unwrap();

        amplifier_c.push_input(output);
        if debug { println!("amplifier C starting"); }
        let output = amplifier_c.run_program(debug).unwrap();

        amplifier_d.push_input(output);
        if debug { println!("amplifier D starting"); }
        let output = amplifier_d.run_program(debug).unwrap();

        amplifier_e.push_input(output);
        if debug { println!("amplifier E starting"); }
        let output = amplifier_e.run_program(debug).unwrap();

        if debug { println!("output: {}", output); }
        if output > max {max = output; println!("{:?}", input);};
    }
    println!("Highest signal that can be sent to the thrusters: {}", max);
}

fn part2() {
    let program = read_data("data");

    let mut data = vec![5, 6, 7, 8, 9];
    let heap = Heap::new(&mut data);

    let mut permutations = Vec::new();
    for data in heap {
        permutations.push(data.clone());
    }

    let mut max = 0;
    let debug = false;
    for input in permutations {
        if debug {
            print!("{:?} ", input);
        };

        let mut amplifier_a = Amplifier::new(program.clone(), vec![input[0],0]); 
        let mut amplifier_b = Amplifier::new(program.clone(), vec![input[1]]); 
        let mut amplifier_c = Amplifier::new(program.clone(), vec![input[2]]); 
        let mut amplifier_d = Amplifier::new(program.clone(), vec![input[3]]); 
        let mut amplifier_e = Amplifier::new(program.clone(), vec![input[4]]); 

        if debug { println!("amplifier A starting"); }
        let mut output = amplifier_a.run_program(debug).unwrap();

        amplifier_b.push_input(output);
        if debug { println!("amplifier B starting"); }
        output = amplifier_b.run_program(debug).unwrap();

        amplifier_c.push_input(output);
        if debug { println!("amplifier C starting"); }
        output = amplifier_c.run_program(debug).unwrap();

        amplifier_d.push_input(output);
        if debug { println!("amplifier D starting"); }
        output = amplifier_d.run_program(debug).unwrap();
        
        amplifier_e.push_input(output);
        if debug { println!("amplifier E starting"); }
        output = match amplifier_e.run_program(debug) {
            Some(num) => num,
            None => { 
                continue
            },
        };
        let mut last_amplifier_output = output;

        loop {
            amplifier_a.push_input(output);
            if debug { println!("amplifier A resuming"); }
            output = match amplifier_a.run_program(debug) {
                Some(num) => num,
                None => break,
            };

            amplifier_b.push_input(output);
            if debug { println!("amplifier B resuming"); }
            output = match amplifier_b.run_program(debug) {
                Some(num) => num,
                None => break,
            };

            amplifier_c.push_input(output);
            if debug { println!("amplifier C resuming"); }
            output = match amplifier_c.run_program(debug) {
                Some(num) => num,
                None => break,
            };

            amplifier_d.push_input(output);
            if debug { println!("amplifier D resuming"); }
            output = match amplifier_d.run_program(debug) {
                Some(num) => num,
                None => break,
            };

            amplifier_e.push_input(output);
            if debug { println!("amplifier E resuming"); }
            output = match amplifier_e.run_program(debug) {
                Some(num) => num,
                None => break,
            };
            last_amplifier_output = output;
        }

        if debug {
            println!("output: {}", last_amplifier_output);
        };
        if last_amplifier_output > max {max = last_amplifier_output;};
    }
    println!("Highest signal that can be sent to the thrusters: {}", max);
}

fn main() {
    part1();
    part2();
}
