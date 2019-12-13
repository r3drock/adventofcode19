extern crate intcomputer;

fn part1() {
    let program = intcomputer::intcode::read_data("data");
    let mut computer = intcomputer::intcode::Amplifier::new(program.clone(), vec![]);
    computer.push_input(1);
    let debug = false;
    computer.run_program(debug).unwrap();
}

fn part2() {
    let program = intcomputer::intcode::read_data("data");
    let mut computer = intcomputer::intcode::Amplifier::new(program.clone(), vec![]);
    computer.push_input(2);
    let debug = false;
    computer.run_program(debug).unwrap();
}


fn main() {
    part2();
}
