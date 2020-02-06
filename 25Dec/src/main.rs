use _25::*;
use intcomputer::intcode;

fn part1() {
    let program = intcode::read_data("program");
    let mut computer = intcode::Amplifier::new(program, vec![]);
    let mut board = Board::new(computer);
    board.start();

    loop {
        board.step();
    }

}

fn main() {
    part1();
}
