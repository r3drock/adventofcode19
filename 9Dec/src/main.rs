use std::io;
use std::fs;
use std::convert::TryFrom;
use std::collections::VecDeque;

#[derive(Debug)]
struct Amplifier {
    ip: usize,
    rb: isize,
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

fn get_modes (mut value: usize) -> Modes {
    value /= 100;
    let mut modes = Modes { mode1: 0, mode2: 0, mode3: 0};
    modes.mode1 = value % 10;
    value /= 10;
    modes.mode2 = value % 10;
    value /= 10;
    modes.mode3 = value % 10;
    modes
}

struct Modes {
    mode1: usize,
    mode2: usize,
    mode3: usize,
}

impl Amplifier {
    fn new(program: Vec<isize>, input: Vec<isize>) -> Amplifier {
        let mut temp = Amplifier { inputbuffer: VecDeque::from(input), program: program, rb : 0,ip : 0 };
        temp.program.append(&mut vec![0;4 * 1024 * 10]);
        temp
    }

    fn push_input(&mut self, input: isize) {
        self.inputbuffer.push_back(input);
    }

    fn access (&self, mode: usize, index: usize) -> isize {
        match mode {
            0 => {self.program[conv(self.program[index])]},
            1 => {self.program[index]},
            2 => {self.program[conv(self.program[index] + self.rb)]},
            _ => panic!("Wrong mode"),
        }
    }

    fn get_access_index(&self, mode: usize, index: usize) -> usize {
        match mode {
            0 => {conv(self.program[index])},
            1 => {index},
            2 => {conv(self.program[conv(index as isize + self.rb)])},
            _ => panic!("Wrong mode"),
        }
    }

    fn pp(&self) {
        print!("[");
        for (i, item) in self.program.iter().enumerate() {
            if i == self.ip {
            print!(">{}<, ", item);
            } else {
            print!("{}, ", item);
            }
            if *item == 0 as isize {break;}
        }
        println!("]");
    }

    fn run_program(&mut self, debug: bool) -> Option<isize> {
        let mut output: Option<isize> = None;
        let size = self.program.len();
        if self.ip >= size { return output;} 
        loop {
            let modes = get_modes(conv(self.program[self.ip]));

            if debug {self.pp()};
            match get_opcode(conv(self.program[self.ip])) {
                //zero
                0 => break,
                //add
                1 => {
                    let index_to_overwrite = self.get_access_index(modes.mode3, self.ip+3);
                    if debug {
                        println!("ADD {}, {}, {}", self.access(modes.mode1, self.ip+1), 
                                                   self.access(modes.mode2, self.ip+2),
                                                   self.program[self.ip+3] );
                    };
                    self.program[index_to_overwrite] = self.access(modes.mode1, self.ip+1) +
                                                  self.access(modes.mode2, self.ip+2);
                    self.ip += 4;
                }

                //mul
                2 => {
                    let index_to_overwrite = self.get_access_index(modes.mode3, self.ip+3);
                    if debug {
                        println!("MUL {}, {}, {}", self.access(modes.mode1, self.ip+1), 
                                                   self.access(modes.mode2, self.ip+2),
                                                   self.program[self.ip+3] );
                    };
                    self.program[index_to_overwrite] = self.access(modes.mode1, self.ip+1) *
                                                  self.access(modes.mode2, self.ip+2);
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

                    let index_to_overwrite = self.get_access_index(modes.mode1, self.ip+1);
                    self.program[index_to_overwrite] = input;
                    self.ip += 2;
                },

                //output
                4 => {
                    output = Some(self.access(modes.mode1, self.ip+1));
                    println!("{:?} ", output.unwrap());
                    if debug { println!("mode: {} outputaddr {} = {}", modes.mode1, self.ip+1, output.unwrap()); };
                    self.ip += 2;
                },

                //jump-if-true
                5 => {
                    self.ip = if self.access(modes.mode1, self.ip+1) != 0 {
                        if debug {
                            println!("jump {}", self.access(modes.mode2, self.ip+2));
                        };
                        conv(self.access(modes.mode2, self.ip+2))
                    } else {
                        if debug {
                            println!("jump {}", self.ip + 3);
                        };
                        self.ip + 3
                    };
                },

                //jump-if-false
                6 => {
                    self.ip = if self.access(modes.mode1, self.ip+1) == 0 {
                        if debug {
                            println!("jump {}", self.access(modes.mode2, self.ip+2));
                        };
                        conv(self.access(modes.mode2, self.ip+2))
                    } else {
                        if debug {
                            println!("jump {}", self.ip + 3);
                        };
                        self.ip + 3
                    };
                },

                //less than
                7 => {
                    let index_to_overwrite = self.get_access_index(modes.mode3, self.ip+3);
                    self.program[index_to_overwrite] =
                        if self.access(modes.mode1, self.ip+1) < self.access(modes.mode2, self.ip+2) {
                        if debug {
                                println!("lt {} {} {}", self.access(modes.mode1, self.ip+1),
                                    self.access(modes.mode1, self.ip+1), index_to_overwrite);
                            };
                        1
                    } else {
                        if debug {
                                println!("not lt {} {} {}", self.access(modes.mode1, self.ip+1),
                                    self.access(modes.mode1, self.ip+1), index_to_overwrite);
                            };
                        0
                    };
                    self.ip += 4;
                }

                //equals
                8 => {
                    let index_to_overwrite = self.get_access_index(modes.mode3, self.ip+3);
                    self.program[index_to_overwrite] =
                        if self.access(modes.mode1, self.ip+1) == self.access(modes.mode2, self.ip+2) {
                            if debug {
                                println!("equals {} {} {}", self.access(modes.mode1, self.ip+1),
                                    self.access(modes.mode1, self.ip+1), index_to_overwrite);
                            };
                            1
                        } else {
                            if debug {
                                println!("not equals {} {} {}", self.access(modes.mode1, self.ip+1),
                                    self.access(modes.mode1, self.ip+1), index_to_overwrite);
                            };
                            0
                        };
                    self.ip += 4;
                }

                //adjust relative base
                9 => {
                    let offset = self.access(modes.mode1, self.ip+1);
                    if debug { println!("adjustrelative base from {} to {}", self.rb, self.rb + offset); };
                    self.rb += offset;
                    self.ip += 2;
                },

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
    let mut computer = Amplifier::new(program.clone(), vec![]); 
    computer.push_input(1);
    let debug = false;
    let output = computer.run_program(debug).unwrap();
    println!("{}", output);
}


fn main() {
    part1();
}
