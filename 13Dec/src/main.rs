extern crate intcomputer;

use std::fmt;

#[derive(Clone, Copy, Eq, PartialEq)]
enum Tile {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

impl Tile {
    fn new(tile_id: isize) -> Tile {
        match tile_id {
            0 => Tile::Empty,
            1 => Tile::Wall,
            2 => Tile::Block,
            3 => Tile::Paddle,
            4 => Tile::Ball,
            _ => panic!("Invalid Tile id."),
        }
    }
}
impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}",
            match self {
                Tile::Empty => 'e',
                Tile::Wall => 'W',
                Tile::Block => 'B',
                Tile::Paddle => 'P',
                Tile::Ball => 'O',
            }
        )
    }
}

fn run_3_instructions(computer: &mut intcomputer::intcode::Amplifier) -> Option<(isize, usize, isize)> {
    let x = match computer.run_program_until_output(false) {
        Some(a) => a,
        None => return None,
    };
    let y = match computer.run_program_until_output(false) {
        Some(a) => a as usize,
        None => return None,
    };
    let tile_id = match computer.run_program_until_output(false) {
        Some(a) => a,
        None => return None,
    };
    Some((x, y, tile_id))
}

fn print_screen(screen: &Vec<Vec<Tile>>, x_len: usize, y_len: usize) {
    for y in 0..y_len {
        for x in 0..x_len {
            print!("{}", screen[y][x]);
        }
        println!();
    }
}

fn count_block_tiles(screen: &Vec<Vec<Tile>>, x_len: usize, y_len: usize) -> usize {
    let mut count = 0;
    for y in 0..y_len {
        for x in 0..x_len {
            if screen[y][x] == Tile::Block {
                count += 1
            }
        }
    }
    count
}

fn part1() {
    let  program = intcomputer::intcode::read_data("program");
    let mut computer = intcomputer::intcode::Amplifier::new(program.clone(), vec![]);
    const X_LEN: usize = 42;
    const Y_LEN: usize = 23;
    let mut screen = vec![vec![Tile::Empty; X_LEN]; Y_LEN];

    while let Some((x, y, tile_id)) = run_3_instructions(&mut computer) {
            screen[y][x as usize] = Tile::new(tile_id);
    }
    print_screen(&screen, X_LEN, Y_LEN);
    println!("{}",count_block_tiles(&screen, X_LEN, Y_LEN));

}

fn part2() {
    let mut program = intcomputer::intcode::read_data("program");
    program[0] = 2;
    let mut computer = intcomputer::intcode::Amplifier::new(program.clone(), vec![]);
    const X_LEN: usize = 42;
    const Y_LEN: usize = 23;
    let mut screen = vec![vec![Tile::Empty; X_LEN]; Y_LEN];

    while let Some((x, y, tile_id)) = run_3_instructions(&mut computer) {
        if x == -1 && y == 0 {
            println!("Score {}", tile_id);
            print_screen(&screen, X_LEN, Y_LEN);
        }
        else {
            screen[y][x as usize] = Tile::new(tile_id);
        }
    }
    print_screen(&screen, X_LEN, Y_LEN);
    println!("{}",count_block_tiles(&screen, X_LEN, Y_LEN));

}

fn main() {
    part1();
}
