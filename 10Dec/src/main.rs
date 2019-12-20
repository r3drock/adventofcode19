use std::fmt;
use std::fs;

#[derive(PartialEq, Copy, Clone, Debug)]
enum Object {
    Asteroid,
    UndetectableAsteroid,
    SpaceStation,
    Empty,
}

#[derive(Copy, Clone, Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Object::Asteroid => '#',
                Object::UndetectableAsteroid => '+',
                Object::SpaceStation => 'O',
                Object::Empty => '.',
            }
        )
    }
}

// Euclid's two-thousand-year-old algorithm for finding the greatest common
// divisor.
fn gcd(x: isize, y: isize) -> isize {
    let mut x = x;
    let mut y = y;
    while y != 0 {
        let t = y;
        y = x % y;
        x = t;
    }
    x
}

fn read_data(file_name: &str) -> (Vec<Vec<Object>>, usize, usize) {
    let data: String =
        fs::read_to_string(file_name).expect("Something went wrong reading the file");
    let x_len = data.lines().next().unwrap().len() as usize;
    let y_len = data.lines().count() as usize;
    let mut space: Vec<Vec<Object>> = vec![vec![Object::Empty; x_len]; y_len];

    for (y, line) in data.split('\n').enumerate() {
        for (x, c) in line.chars().enumerate() {
            space[y][x] = match c {
                '#' => Object::Asteroid,
                '.' => Object::Empty,
                e => panic!("Wrong symbol {}", e),
            };
        }
    }
    (space, x_len, y_len)
}

fn print_space(space: &Vec<Vec<Object>>, x_len: usize, y_len: usize) {
    for y in 0..y_len {
        for x in 0..x_len {
            print!("{}", space[y][x]);
        }
        println!();
    }
}

fn mark(space: &mut Vec<Vec<Object>>, station_pos: Point, pos: Point, x_len: isize, y_len: isize) {
    space[station_pos.y][station_pos.x] = Object::SpaceStation;
    let mut curr_x_pos = pos.x as isize;
    let mut curr_y_pos = pos.y as isize;
    let x_diff = pos.x as isize - station_pos.x as isize;
    let y_diff = pos.y as isize - station_pos.y as isize;

    let gcd = gcd((x_diff).abs(), (y_diff).abs());

    let x_diff= x_diff / gcd;
    let y_diff= y_diff / gcd;

    curr_x_pos += x_diff;
    curr_y_pos += y_diff;

    while curr_x_pos >= 0 && curr_x_pos < x_len && curr_y_pos >= 0 && curr_y_pos < y_len {
        let field = &mut space[curr_y_pos as usize][curr_x_pos as usize];
        *field = if *field == Object::Asteroid {Object::UndetectableAsteroid} else {*field};
        curr_x_pos += x_diff;
        curr_y_pos += y_diff;
    }
}

fn mark_undetectable_asteroids(
    space: &Vec<Vec<Object>>,
    station_pos: Point,
    x_len: usize,
    y_len: usize,
) -> Vec<Vec<Object>> {
    let mut space_with_undetectable = space.clone();

    for y in 0..y_len {
        for x in 0..x_len {
            if station_pos.x == x && station_pos.y == y {
                continue;
            };
            if space[y][x] == Object::Asteroid {
                mark(
                    &mut space_with_undetectable,
                    station_pos,
                    Point { x: x, y: y },
                    x_len as isize,
                    y_len as isize,
                );
            }
        }
    }
    space_with_undetectable
}

fn count_detectable_asteroids(space: &Vec<Vec<Object>>, x_len: usize, y_len: usize) -> usize {
    let mut count = 0;
    for y in 0..y_len {
        for x in 0..x_len {
            count += if space[y][x] == Object::Asteroid {
                1
            } else {
                0
            };
        }
    }
    count
}

fn get_ideal_number_of_detectable_asteroids(space: &Vec<Vec<Object>>, x_len: usize, y_len: usize) -> usize {
    let mut max = 0;
    for y in 0..y_len {
        for x in 0..x_len {
            if space[y][x] == Object::Asteroid {
                let result_space = mark_undetectable_asteroids(&space, Point { x: x, y: y }, x_len, y_len);
                let result= count_detectable_asteroids(&result_space, x_len, y_len);
                if result > max {
                    max = result;
                }
            }
        }
    }
    max
}

fn part1() {
    let (space, x_len, y_len) = read_data("data");
    println!("Ideal Number: {}", get_ideal_number_of_detectable_asteroids(&space, x_len, y_len));
}

fn main() {
    part1();
}
