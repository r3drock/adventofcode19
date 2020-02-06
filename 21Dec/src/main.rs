use intcomputer::intcode;
use std::collections::VecDeque;
use std::fs;
use std::convert::TryInto;

fn read_spring_script(path: &str) -> Vec<isize> {
    let mut script = Vec::new();

    let data = fs::read_to_string(path)
        .expect("Something went wrong reading the file");

    for c in data.chars() {
        script.push(c as isize);
    }
    script
}

fn print_output(computer: &mut intcode::Amplifier) {
    while let Some(output) = computer.run_program_until_output(false) {
        match output > 255 {
            true => {println!("{}", output)},
            false => {print!("{}", output as u8 as char)},
        }
    }
}

fn part1() {
    let program = intcode::read_data("program");
    let script = read_spring_script("script");
    let mut computer = intcode::Amplifier::new(program, vec![]);
    computer.push_input_vec(script);
    print_output(&mut computer);
}

fn main() {
    part1();

}
