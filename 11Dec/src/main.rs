extern crate intcomputer;
use std::fmt;

const X_LEN: isize = 200;
const Y_LEN: isize = 80;

#[derive(Copy,Clone)]
enum Color {
    Black,
    White,
    Unpainted,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}",
               match self {
                   Color::Unpainted => {' '}
                   Color::White => {'#'}
                   Color::Black => {'.'}
               }
        )
    }
}

struct Point {
    x: isize,
    y: isize,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}",
               match self {
                   Direction::Up => {'^'},
                   Direction::Down => {'v'},
                   Direction::Left => {'<'},
                   Direction::Right => {'>'},
               }
        )
    }
}

fn set_at(field: &mut Vec<Vec<Color>>, pos: &Point, val: Color) {
    field[(pos.y+Y_LEN/2) as usize][(pos.x+X_LEN/2) as usize] = val;
}

fn get_at(field: & Vec<Vec<Color>>, pos: &Point) -> Color {
    field[(pos.y+Y_LEN/2) as usize][(pos.x+X_LEN/2) as usize]
}


fn print_field(field: &Vec<Vec<Color>>, pos: &Point, dir: &Direction) {
    for y in 0..Y_LEN {
        for x in 0..X_LEN {
            let string_to_print =
                if pos.x == x && pos.y == y {
                    format!("{}", dir)
                } else {
                format!("{}", get_at(&field, &Point {x: x - (X_LEN/2), y: y - (Y_LEN/2)}))
        };
        print!("{}", string_to_print);
        }
    println!("");
    }
}

fn part1() {
    let program = intcomputer::intcode::read_data("input");
    let mut computer = intcomputer::intcode::Amplifier::new(program.clone(), vec![]);
    let mut field: Vec<Vec<Color>> = vec![vec![Color::Unpainted;X_LEN as usize];Y_LEN as usize];
    let mut pos = Point {x : 0, y : 0};

    let mut dir: Direction = Direction::Up;
//    print_field(&field, &pos, &dir);


    loop {
        computer.push_input(
            match get_at(&field, &pos) {
                Color::Unpainted => 0,
                Color::Black => 0,
                Color::White => 1,
            });
        let output =
            match computer.run_program_until_output(false) {
                Some(result) => result,
                None => break,
            };
        set_at(&mut field, &pos, match output {
            0 => Color::Black,
            1 => Color::White,
            _ => panic!("invalid output"),
        });

        let output =
            match computer.run_program_until_output(false) {
                Some(result) => result,
                None => break,
            };
        dir = turn(dir, output);
        pos = move_forward(&dir, pos);
    }
 //   print_field(&field, &pos, &dir);
    print_field(&field, &pos, &dir);
    println!("{}", count_fields_painted_at_least_once(field));
}

fn count_fields_painted_at_least_once(field: Vec<Vec<Color>>) -> usize {
    let mut count = 0;
    for row in field.iter() {
        for field in row.iter() {
            count += match field {
                Color::Unpainted => 0,
                _ => 1,
            }
        }
    }
    count
}

fn turn (dir: Direction, val: isize) -> Direction {
    match (dir, val) {
        (Direction::Up, 0) => Direction::Left,
        (Direction::Up, 1) => Direction::Right,
        (Direction::Down, 0) => Direction::Right,
        (Direction::Down, 1) => Direction::Left,
        (Direction::Left, 0) => Direction::Down,
        (Direction::Left, 1) => Direction::Up,
        (Direction::Right, 0) => Direction::Up,
        (Direction::Right, 1) => Direction::Down,
        (_,_) => panic!("Wrong direction to turn to"),
    }
}

fn move_forward (dir: &Direction, pos: Point) -> Point {
    match dir {
        Direction::Up => Point {x: pos.x, y: pos.y - 1},
        Direction::Down => Point {x: pos.x, y: pos.y + 1},
        Direction::Left => Point {x: pos.x - 1, y: pos.y},
        Direction::Right => Point {x: pos.x + 1, y: pos.y},
    }
}

fn part2() {
    let program = intcomputer::intcode::read_data("input");
    let mut computer = intcomputer::intcode::Amplifier::new(program.clone(), vec![]);
    let mut field: Vec<Vec<Color>> = vec![vec![Color::Unpainted;X_LEN as usize];Y_LEN as usize];
    let mut pos = Point {x : 0, y : 0};
    set_at(&mut field, &pos, Color::White);

    let mut dir: Direction = Direction::Up;
//    print_field(&field, &pos, &dir);


    loop {
        computer.push_input(
            match get_at(&field, &pos) {
                Color::Unpainted => 0,
                Color::Black => 0,
                Color::White => 1,
            });
        let output =
            match computer.run_program_until_output(false) {
                Some(result) => result,
                None => break,
            };
        set_at(&mut field, &pos, match output {
            0 => Color::Black,
            1 => Color::White,
            _ => panic!("invalid output"),
        });

        let output =
            match computer.run_program_until_output(false) {
                Some(result) => result,
                None => break,
            };
        dir = turn(dir, output);
        pos = move_forward(&dir, pos);
    }
    print_field(&field, &pos, &dir);
}

fn main() {
    part1();
    part2();
}
