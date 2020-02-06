pub mod intcode {
    use std::collections::VecDeque;
    use std::sync::mpsc;
    use std::convert::TryFrom;
    use std::{fs, thread};
    use std::time::Duration;
    use std::thread::Thread;

    #[derive(Debug)]
    pub struct NetworkAmplifier {
        ip: usize,
        rb: isize,
        tx: mpsc::Sender<(isize,isize)>,
        rx: mpsc::Receiver<(isize,isize)>,
        program: Vec<isize>,
        y: Option<isize>,
    }

    #[derive(Copy, Clone)]
    pub enum Instruction {
        NOOP,
        ADD(usize, usize, usize),
        MULT(usize, usize, usize),
        READ(usize),
        WRITE(usize),
        JUMPIFTRUE(usize, usize),
        JUMPIFFALSE(usize, usize),
        LESSTHAN(usize, usize, usize),
        EQUALS(usize, usize, usize),
        ADJUSTRB(usize),
        HALT,
    }

    impl Instruction {
        fn first(self) -> usize {
            match self {
                Instruction::NOOP => panic!("NOOP has no first parameter"),
                Instruction::ADD(a, _, _) => a,
                Instruction::MULT(a, _, _) => a,
                Instruction::READ(_) => panic!("READ has no first parameter"),
                Instruction::WRITE(a) => a,
                Instruction::JUMPIFTRUE(a, _) => a,
                Instruction::JUMPIFFALSE(a, _) => a,
                Instruction::LESSTHAN(a, _, _) => a,
                Instruction::EQUALS(a, _, _) => a,
                Instruction::ADJUSTRB(a) => a,
                Instruction::HALT => panic!("HALT should never run"),
            }
        }
        fn second(self) -> usize {
            match self {
                Instruction::NOOP => panic!("NOOP has no first parameter"),
                Instruction::ADD(_, b, _) => b,
                Instruction::MULT(_, b, _) => b,
                Instruction::READ(_) => panic!("READ has no second parameter"),
                Instruction::WRITE(_) => panic!("WRITE has no second parameter"),
                Instruction::JUMPIFTRUE(_, b) => b,
                Instruction::JUMPIFFALSE(_, b) => b,
                Instruction::LESSTHAN(_, b, _) => b,
                Instruction::EQUALS(_, b, _) => b,
                Instruction::ADJUSTRB(_) => panic!("ADJUSTRB has no second parameter"),
                Instruction::HALT => panic!("HALT should never run"),
            }
        }
        fn target(self) -> usize {
            match self {
                Instruction::NOOP => panic!("NOOP has no target"),
                Instruction::ADD(_, _, c) => c,
                Instruction::MULT(_, _, c) => c,
                Instruction::READ(a) => a,
                Instruction::WRITE(_) => panic!("WRITE has no target"),
                Instruction::JUMPIFTRUE(_, _) => panic!("JUMPIFTRUE has no target"),
                Instruction::JUMPIFFALSE(_, _) => panic!("JUMPIFTRUE has no target"),
                Instruction::LESSTHAN(_, _, c) => c,
                Instruction::EQUALS(_, _, c) => c,
                Instruction::ADJUSTRB(_) => panic!("ADJUSTRB has no target"),
                Instruction::HALT => panic!("HALT should never run"),
            }
        }
    }

    pub fn read_data(file_name: &str) -> Vec<isize> {
        let mut program: Vec<isize> = Vec::new();
        let data = fs::read_to_string(file_name).expect("Something went wrong reading the file");
        for line in data.split(",") {
            match line.parse::<isize>() {
                Ok(x) => program.push(x),
                Err(_) => (),
            };
        }
        program
    }

    fn conv(x: isize) -> usize {
        usize::try_from(x).expect(&format!("negative instruction pointer {}",x))
    }

    fn get_opcode(mut value: usize) -> usize {
        let opcode = value % 10;
        value /= 10;
        opcode + (value % 10) * 10
    }

    fn into_mode(value: usize) -> Mode {
        match value {
            0 => Mode::Position,
            1 => Mode::Immediate,
            2 => Mode::Relative,
            _ => panic!("invalid mode"),
        }
    }

    #[derive(Debug, PartialEq)]
    pub enum Mode {
        Position,
        Immediate,
        Relative,
    }

    #[derive(Debug, PartialEq)]
    pub struct Modes {
        pub mode1: Mode,
        pub mode2: Mode,
        pub mode3: Mode,
    }

    pub fn get_modes(mut value: usize) -> Modes {
        value /= 100;
        let m1 = into_mode(value % 10);
        value /= 10;
        let m2 = into_mode(value % 10);
        value /= 10;
        let m3 = into_mode(value % 10);
        if m3 == Mode::Immediate {
            panic!("Parameters that an instruction writes to must never be in immediate mode.");
        }
        Modes {
            mode1: m1,
            mode2: m2,
            mode3: m3,
        }
    }

    impl NetworkAmplifier {
        pub fn new(program: Vec<isize>) -> NetworkAmplifier {
            let (tx, rx) = mpsc::channel();
            let mut temp = NetworkAmplifier {
                program: program,
                tx: tx,
                rx: rx,
                rb: 0,
                ip: 0,
                y: None,
            };
            temp.program.append(&mut vec![0; 4 * 1024 * 1000]);
            temp
        }
        pub fn new_test(program: Vec<isize>) -> NetworkAmplifier {
            let (tx, rx) = mpsc::channel();
            NetworkAmplifier {
                program: program,
                tx: tx,
                rx: rx,
                rb: 0,
                ip: 0,
                y: None,
            }
        }


        pub fn get_input_transmitter(&self) -> mpsc::Sender<(isize,isize)> {
            self.tx.clone()
        }

        pub fn push_input(&mut self, input: (isize, isize)) {
            self.tx.send(input).unwrap();
            let a = 1 +1;
        }

        pub fn push_input_vec(&mut self, input: Vec<(isize,isize)>) {
            for i in input {
                self.tx.send(i).unwrap();
            }
        }

        pub fn push_input_vec_deque(&mut self, input: VecDeque<(isize, isize)>) {
            for i in input {
                self.tx.send(i).unwrap();
            }
        }

        fn get_access_index(&self, mode: Mode, index: usize) -> usize {
            match mode {
                Mode::Position => conv(self.program[index]),
                Mode::Immediate => index,
                Mode::Relative => conv(self.program[index] + self.rb),
            }
        }

        pub fn print_program(&self) {
            print!("[");
            let mut was_zero = 0;
            for (i, item) in self.program.iter().enumerate() {
                if i == self.ip {
                    print!(">{}<, ", item);
                } else {
                    print!("{}, ", item);
                }
                if *item == 0 as isize {
                    if was_zero == 50{
                        break;
                    } else {
                        was_zero += 1;
                    }
                } else {
                    was_zero = 0;
                }
            }
            println!("]");
        }

        fn parse_instruction(&self) -> Instruction {
            let opcode = get_opcode(conv(self.program[self.ip]));
            let modes = get_modes(conv(self.program[self.ip]));
            let inst = match opcode {
                0 => Instruction::NOOP,
                1 => Instruction::ADD(
                    self.get_access_index(modes.mode1, self.ip + 1),
                    self.get_access_index(modes.mode2, self.ip + 2),
                    self.get_access_index(modes.mode3, self.ip + 3),
                ),
                2 => Instruction::MULT(
                    self.get_access_index(modes.mode1, self.ip + 1),
                    self.get_access_index(modes.mode2, self.ip + 2),
                    self.get_access_index(modes.mode3, self.ip + 3),
                ),
                3 => Instruction::READ(self.get_access_index(modes.mode1, self.ip + 1)),
                4 => Instruction::WRITE(self.get_access_index(modes.mode1, self.ip + 1)),
                5 => Instruction::JUMPIFTRUE(
                    self.get_access_index(modes.mode1, self.ip + 1),
                    self.get_access_index(modes.mode2, self.ip + 2),
                ),
                6 => Instruction::JUMPIFFALSE(
                    self.get_access_index(modes.mode1, self.ip + 1),
                    self.get_access_index(modes.mode2, self.ip + 2),
                ),
                7 => Instruction::LESSTHAN(
                    self.get_access_index(modes.mode1, self.ip + 1),
                    self.get_access_index(modes.mode2, self.ip + 2),
                    self.get_access_index(modes.mode3, self.ip + 3),
                ),
                8 => Instruction::EQUALS(
                    self.get_access_index(modes.mode1, self.ip + 1),
                    self.get_access_index(modes.mode2, self.ip + 2),
                    self.get_access_index(modes.mode3, self.ip + 3),
                ),
                9 => Instruction::ADJUSTRB(self.get_access_index(modes.mode1, self.ip + 1)),
                99 => Instruction::HALT,
                a => panic!("Illegal Opcode {}", a),
            };
            inst
        }

        pub fn add(&mut self, inst: Instruction, debug: bool) {
            if debug {
                println!("[{}] := [{}] + [{}]\n {} = {} + {}", inst.target(), inst.first(), inst.second(),
                self.program[inst.first()] + self.program[inst.second()], self.program[inst.first()], self.program[inst.second()]);
            };
            self.program[inst.target()] = self.program[inst.first()] + self.program[inst.second()];
            self.ip += 4;
        }

        pub fn mult(&mut self, inst: Instruction, debug: bool) {
            if debug {
                println!(
                    "MULT {}, {}, {}",
                    inst.target(),
                    inst.first(),
                    inst.second()
                );
            };
            self.program[inst.target()] = self.program[inst.first()] * self.program[inst.second()];
            self.ip += 4;
        }

        fn read(&mut self, inst: Instruction, debug: bool) {
            let a = 1 + 1;
            let input: isize = match self.y {
                Some(y) => {self.y = None; y},
                None => {
                    match self.rx.try_recv() {
                        Ok((x, y)) => {
                            if debug { println!("input {}", x); }
                            self.y = Some(y);
                            x
                        },
                        Err(_) => {
                            -1
                        },
                    }
                },
            };
            self.program[inst.target()] = input;
            self.ip += 2;
        }

        fn write(&mut self, inst: Instruction, debug: bool) -> isize {
            let output = self.program[inst.first()];
            if debug {
                println!("outputaddr {} = {}", inst.first(), output);
            };
            //println!("{}", output);
            self.ip += 2;
            output
        }

        fn jump_if_true(&mut self, inst: Instruction, debug: bool) {
            if debug {
                println!("jump if [{}] containing [{}]", inst.first(), self.program[inst.first()]);
            };
            self.ip = if self.program[inst.first()] != 0 {
                if debug {
                    println!(" jump to {}", self.program[inst.second()]);
                }
                conv(self.program[inst.second()])
            } else {
                if debug {
                    println!(" jump to {}", self.ip + 3);
                }
                self.ip + 3
            };
        }

        fn jump_if_false(&mut self, inst: Instruction, debug: bool) {
            self.ip = if self.program[inst.first()] == 0 {
                if debug {
                    println!("jump if false {}", self.program[inst.second()]);
                };
                conv(self.program[inst.second()])
            } else {
                if debug {
                    println!("not jump if false {}", self.ip + 3);
                };
                self.ip + 3
            };
        }

        fn less_than(&mut self, inst: Instruction, debug: bool) {
            self.program[inst.target()] =
                if self.program[inst.first()] < self.program[inst.second()] {
                    if debug {
                        println!(
                            "lt {} {} {}",
                            self.program[inst.first()],
                            self.program[inst.second()],
                            self.program[inst.target()]
                        );
                    };
                    1
                } else {
                    if debug {
                        println!(
                            "not lt {} {} {}",
                            self.program[inst.first()],
                            self.program[inst.second()],
                            self.program[inst.target()]
                        );
                    };
                    0
                };
            self.ip += 4;
        }

        fn equals(&mut self, inst: Instruction, debug: bool) {
            if debug {
                println!(
                    "[{}] := [{}] == [{}]",
                    inst.target(),
                    inst.first(),
                    inst.second()
                );
            }
            self.program[inst.target()] =
                if self.program[inst.first()] == self.program[inst.second()] {
                    if debug {
                        println!(
                            " {} == {}",
                            self.program[inst.first()],
                            self.program[inst.second()],
                        );
                    };
                    1
                } else {
                    if debug {
                        println!(
                            " {} != {}",
                            self.program[inst.first()],
                            self.program[inst.second()],
                        );
                    };
                    0
                };
            self.ip += 4;
        }

        fn adjust_rb(&mut self, inst: Instruction, debug: bool) {
            let offset = self.program[inst.first()];
            if debug {
                println!(
                    "adjust relative base from {} to {}",
                    self.rb,
                    self.rb + offset
                );
            };
            self.rb += offset;
            self.ip += 2;
        }

        pub fn run_program(&mut self, debug: bool) -> Option<isize> {
            let mut output: Option<isize> = None;
            let size = self.program.len();
            if self.ip >= size {
                return output;
            }
            loop {
                let inst = self.parse_instruction();
                if debug {
                    self.print_program();
                    println!("[{}]:", self.ip);
                };
                match inst {
                    Instruction::NOOP => panic!("NOOP"),
                    Instruction::ADD(_, _, _) => self.add(inst, debug),
                    Instruction::MULT(_, _, _) => self.mult(inst, debug),
                    Instruction::READ(_) => self.read(inst, debug),
                    Instruction::WRITE(_) => output = Some(self.write(inst, debug)),
                    Instruction::JUMPIFTRUE(_, _) => self.jump_if_true(inst, debug),
                    Instruction::JUMPIFFALSE(_, _) => self.jump_if_false(inst, debug),
                    Instruction::LESSTHAN(_, _, _) => self.less_than(inst, debug),
                    Instruction::EQUALS(_, _, _) => self.equals(inst, debug),
                    Instruction::ADJUSTRB(_) => self.adjust_rb(inst, debug),
                    Instruction::HALT => {
                        if debug {
                            println!("END")
                        };
                        break;
                    }
                }
            }
            output
        }

        pub fn run_program_until_output(&mut self, debug: bool) -> Option<isize> {
            let mut output: Option<isize> = None;
            let size = self.program.len();
            if self.ip >= size {
                return output;
            }
            loop {
                let inst = self.parse_instruction();
                if debug {
                    self.print_program()
                };
                print!("");
                match inst {
                    Instruction::NOOP => continue,
                    Instruction::ADD(_, _, _) => self.add(inst, debug),
                    Instruction::MULT(_, _, _) => self.mult(inst, debug),
                    Instruction::READ(_) => self.read(inst, debug),
                    Instruction::WRITE(_) => {output = Some(self.write(inst, debug)); break;},
                    Instruction::JUMPIFTRUE(_, _) => self.jump_if_true(inst, debug),
                    Instruction::JUMPIFFALSE(_, _) => self.jump_if_false(inst, debug),
                    Instruction::LESSTHAN(_, _, _) => self.less_than(inst, debug),
                    Instruction::EQUALS(_, _, _) => self.equals(inst, debug),
                    Instruction::ADJUSTRB(_) => self.adjust_rb(inst, debug),
                    Instruction::HALT => {
                        if debug {
                            println!("END")
                        };
                        break;
                    }
                }
            }
            output
        }

        pub fn run_program_in_compatibility_mode(&mut self, noun: isize, verb: isize, debug: bool) -> isize {
            self.program[1] = noun;
            self.program[2] = verb;
            loop {
                let inst = self.parse_instruction();
                if debug {
                    self.print_program()
                };
                print!("");
                match inst {
                    Instruction::NOOP => continue,
                    Instruction::ADD(_, _, _) => self.add(inst, debug),
                    Instruction::MULT(_, _, _) => self.mult(inst, debug),
                    Instruction::HALT => {
                        if debug {
                            println!("END")
                        };
                        break;
                    }
                    _ => panic!("instruction not supported in compatibility mode")
                }
            }
            self.program[0]
        }
        
        pub fn test_run(&mut self, debug: bool) {
                let inst = self.parse_instruction();
                if debug {
                    self.print_program()
                };
                match inst {
                    Instruction::NOOP => (),
                    Instruction::ADD(_, _, _) => self.add(inst, debug),
                    Instruction::MULT(_, _, _) => self.mult(inst, debug),
                    Instruction::READ(_) => self.read(inst, debug),
                    Instruction::WRITE(_) => {self.write(inst, debug);()},
                    Instruction::JUMPIFTRUE(_, _) => self.jump_if_true(inst, debug),
                    Instruction::JUMPIFFALSE(_, _) => self.jump_if_false(inst, debug),
                    Instruction::LESSTHAN(_, _, _) => self.less_than(inst, debug),
                    Instruction::EQUALS(_, _, _) => self.equals(inst, debug),
                    Instruction::ADJUSTRB(_) => self.adjust_rb(inst, debug),
                    Instruction::HALT => {
                        if debug {
                            println!("END")
                        };
                        ()
                    },
                }
        }
        pub fn get_program_clone(&self) -> Vec<isize> {
            self.program.clone()
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn day23_part1() {
        let program = crate::intcode::read_data("23");
        let mut computer = crate::intcode::NetworkAmplifier::new(program);
        computer.push_input((34, -1));
        let debug = true;
        assert_eq!(46470, computer.run_program(debug).unwrap());
    }
}

