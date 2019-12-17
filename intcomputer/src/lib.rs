pub mod intcode {
    use std::collections::VecDeque;
    use std::convert::TryFrom;
    use std::fs;
    use std::io;
    #[derive(Debug)]
    pub struct Amplifier {
        ip: usize,
        rb: isize,
        inputbuffer: VecDeque<isize>,
        program: Vec<isize>,
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

    impl Amplifier {
        pub fn new(program: Vec<isize>, input: Vec<isize>) -> Amplifier {
            let mut temp = Amplifier {
                inputbuffer: VecDeque::from(input),
                program: program,
                rb: 0,
                ip: 0,
            };
            temp.program.append(&mut vec![0; 4 * 1024 * 10]);
            temp
        }
        pub fn new_test(program: Vec<isize>, input: Vec<isize>) -> Amplifier {
            Amplifier {
                inputbuffer: VecDeque::from(input),
                program: program,
                rb: 0,
                ip: 0,
            }
        }

        pub fn push_input(&mut self, input: isize) {
            self.inputbuffer.push_back(input);
        }

        fn get_access_index(&self, mode: Mode, index: usize) -> usize {
            match mode {
                Mode::Position => conv(self.program[index]),
                Mode::Immediate => index,
                Mode::Relative => conv(self.program[index] + self.rb),
            }
        }

        fn print_program(&self) {
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
                println!("ADD {}, {}, {}", inst.target(), inst.first(), inst.second());
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
            let input: isize = match self.inputbuffer.pop_front() {
                Some(num) => {
                    if debug {
                        println!("input {}", num);
                    };
                    num
                }
                None => {
                    let mut input_string = String::new();
                    println!("Please input a number.");
                    io::stdin()
                        .read_line(&mut input_string)
                        .expect("error reading line");

                    match input_string.trim().parse() {
                        Ok(num) => num,
                        Err(_) => return,
                    }
                }
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
            self.ip = if self.program[inst.first()] != 0 {
                if debug {
                    println!("jump if true {}", self.program[inst.second()]);
                };
                conv(self.program[inst.second()])
            } else {
                if debug {
                    println!("not jump if true {}", self.ip + 3);
                };
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
            self.program[inst.target()] =
                if self.program[inst.first()] == self.program[inst.second()] {
                    if debug {
                        println!(
                            "equals {} {} {}",
                            self.program[inst.first()],
                            self.program[inst.second()],
                            self.program[inst.target()]
                        );
                    };
                    1
                } else {
                    if debug {
                        println!(
                            "not equals {} {} {}",
                            self.program[inst.first()],
                            self.program[inst.second()],
                            self.program[inst.target()]
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
                    self.print_program()
                };
                print!("");
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
    fn compatibility() {
          let program = crate::intcode::read_data("2");
          let mut computer = crate::intcode::Amplifier::new(program.clone(), vec![]);
          let debug = false;
          assert_eq!(3516593, computer.run_program_in_compatibility_mode(12, 2, debug));
    }
    #[test]
    fn day5_part1() {
        let program = crate::intcode::read_data("5");
        let mut computer = crate::intcode::Amplifier::new(program.clone(), vec![]);
        computer.push_input(1);
        assert_eq!(15508323, computer.run_program(false).unwrap());
    }

    #[test]
    fn test_add_positional() {
        let program = vec![1,2,3,0,4,0,99];
        let mut computer = crate::intcode::Amplifier::new_test(program, vec![]);
        computer.test_run(false);
        assert_eq!(vec![3,2,3,0,4,0,99], computer.get_program_clone());
    }


    #[test]
    fn test_add_immediate() {
        let program = vec![01101,2,3,0,4,0,99];
        let mut computer = crate::intcode::Amplifier::new_test(program, vec![]);
        computer.test_run(false);
        assert_eq!(vec![5,2,3,0,4,0,99], computer.get_program_clone());
    }

    #[test]
    fn test_get_modes() {
        use crate::intcode::Mode;
        let modes = 
            crate::intcode::Modes {
                mode1: Mode::Relative,
                mode2: Mode::Immediate,
                mode3: Mode::Position,
            };
        assert_eq!(modes ,crate::intcode::get_modes(01201));
    }

    #[test]
    #[should_panic]
    fn test_get_modes_invalid() {
        crate::intcode::get_modes(10001);
    }

    #[test]
    fn test_add_relative() {
        let program = vec![01109,2,02201,2,6,0,4,0,99];
        let mut computer = crate::intcode::Amplifier::new(program, vec![]);
        assert_eq!(105, computer.run_program(false).unwrap());
    }

    #[test]
    fn test_jump_if_true_false() {
        let program = vec![01105,0,6,00004,1,99,00004,2,99];
        let mut computer = crate::intcode::Amplifier::new(program, vec![]);
        assert_eq!(0, computer.run_program(false).unwrap());
    }

    #[test]
    fn test_jump_if_true_true() {
        let program = vec![01105,1,6,00004,1,99,00004,2,99];
        let mut computer = crate::intcode::Amplifier::new(program, vec![]);
        assert_eq!(6, computer.run_program(false).unwrap());
    }

    #[test]
    fn test_jump_if_true_immediate_false() {
        let program = vec![01105,0,6,00104,1,99,00104,2,99];
        let mut computer = crate::intcode::Amplifier::new(program, vec![]);
        assert_eq!(1, computer.run_program(false).unwrap());
    }

    #[test]
    fn test_jump_if_false_false() {
        let program = vec![01106,0,6,00004,1,99,00004,2,99];
        let mut computer = crate::intcode::Amplifier::new(program, vec![]);
        assert_eq!(6, computer.run_program(false).unwrap());
    }

    #[test]
    fn test_jump_if_false_true() {
        let program = vec![01106,1,6,00004,1,99,00004,2,99];
        let mut computer = crate::intcode::Amplifier::new(program, vec![]);
        assert_eq!(1, computer.run_program(false).unwrap());
    }

    #[test]
    fn test_jump_if_false_immediate_false() {
        let program = vec![01106,0,6,00104,1,99,00104,2,99];
        let mut computer = crate::intcode::Amplifier::new(program, vec![]);
        assert_eq!(2, computer.run_program(false).unwrap());
    }

    #[test]
    fn test_jump_if_false_immediate_true() {
        let program = vec![01106,1,6,00104,1,99,00104,2,99];
        let mut computer = crate::intcode::Amplifier::new(program, vec![]);
        assert_eq!(1, computer.run_program(false).unwrap());
    }

    #[test]
    fn test_small_5_equal_position_false() {
        let program = vec![3,9,8,9,10,9,4,9,99,-1,8];
        let mut computer = crate::intcode::Amplifier::new(program, vec![]);
        computer.push_input(1);
        assert_eq!(0, computer.run_program(false).unwrap());
    }

    #[test]
    fn test_small_5_equal_position_true() {
        let program = vec![3,9,8,9,10,9,4,9,99,-1,8];
        let mut computer = crate::intcode::Amplifier::new(program, vec![]);
        computer.push_input(8);
        assert_eq!(1, computer.run_program(false).unwrap());
    }

    #[test]
    fn test_small_5_less_than_position_false() {
        let program = vec![3,9,7,9,10,9,4,9,99,-1,8];
        let mut computer = crate::intcode::Amplifier::new_test(program, vec![]);
        computer.push_input(8);
        let result = computer.run_program(false).unwrap();
        println!("{:?}", computer.get_program_clone());
        assert_eq!(0, result);
    }

    #[test]
    fn test_small_5_less_than_position_true() {
        let program = vec![3,9,7,9,10,9,4,9,99,-1,8];
        let mut computer = crate::intcode::Amplifier::new_test(program, vec![]);
        computer.push_input(1);
        let result = computer.run_program(false).unwrap();
        println!("{:?}", computer.get_program_clone());
        assert_eq!(1, result);
    }

    #[test]
    fn test_small_5_equal_immediate_false() {
        let program = vec![3,3,1108,-1,8,3,4,3,99];
        let mut computer = crate::intcode::Amplifier::new_test(program, vec![]);
        computer.push_input(1);
        assert_eq!(0, computer.run_program(false).unwrap());
    }

    #[test]
    fn test_small_5_equal_immediate_true() {
        let program = vec![3,3,1108,-1,8,3,4,3,99];
        let mut computer = crate::intcode::Amplifier::new(program, vec![]);
        computer.push_input(8);
        assert_eq!(1, computer.run_program(false).unwrap());
    }

    #[test]
    fn test_small_5_less_than_immediate_false() {
        let program = vec![3,3,1107,-1,8,3,4,3,99];
        let mut computer = crate::intcode::Amplifier::new_test(program, vec![]);
        computer.push_input(8);
        let result = computer.run_program(false).unwrap();
        println!("{:?}", computer.get_program_clone());
        assert_eq!(0, result);
    }

    #[test]
    fn test_small_5_less_than_immediate_true() {
        let program = vec![3,3,1107,-1,8,3,4,3,99];
        let mut computer = crate::intcode::Amplifier::new(program, vec![]);
        computer.push_input(1);
        assert_eq!(1, computer.run_program(false).unwrap());
    }

    #[test]
    fn day5_part2() {
        let program = crate::intcode::read_data("5");
        let mut computer = crate::intcode::Amplifier::new(program.clone(), vec![]);
        computer.push_input(5);
        assert_eq!(9006327, computer.run_program(false).unwrap());
    }

    #[test]
    fn day7_part1() {
        let program = crate::intcode::read_data("7");

        let mut amp_a = crate::intcode::Amplifier::new_test(program.clone(), vec![4]);
        amp_a.push_input(0);
        let output = amp_a.run_program(false).unwrap();
        let mut amp_b = crate::intcode::Amplifier::new_test(program.clone(), vec![0]);
        amp_b.push_input(output);
        let output = amp_b.run_program(false).unwrap();
        let mut amp_c = crate::intcode::Amplifier::new_test(program.clone(), vec![2]);
        amp_c.push_input(output);
        let output = amp_c.run_program(false).unwrap();
        let mut amp_d = crate::intcode::Amplifier::new_test(program.clone(), vec![3]);
        amp_d.push_input(output);
        let output = amp_d.run_program(false).unwrap();
        let mut amp_e = crate::intcode::Amplifier::new_test(program.clone(), vec![1]);
        amp_e.push_input(output);
        let result = amp_e.run_program(false).unwrap();
        assert_eq!(11828, result);
    }

    #[test]
    fn day9_part1() {
        let program = crate::intcode::read_data("9");
        let mut computer = crate::intcode::Amplifier::new(program, vec![1]);
        let result = computer.run_program(false).unwrap();
        assert_eq!(3497884671, result);
    }

    #[test]
    fn day9_part2() {
        let program = crate::intcode::read_data("9");
        let mut computer = crate::intcode::Amplifier::new(program, vec![2]);
        let result = computer.run_program(false).unwrap();
        assert_eq!(46470, result);
    }

}
