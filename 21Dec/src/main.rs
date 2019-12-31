use intcomputer::intcode;

fn main() {
    let program = intcode::read_data("program");
    let computer = intcode::Amplifier::new(program, vec![]);

}
