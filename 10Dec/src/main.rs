extern crate num_rational;

use std::fmt;
use num_rational::Rational;
use std::cmp::Ordering;
use std::fs;
use std::collections::{HashSet, VecDeque};

#[derive(PartialEq, Copy, Clone, Debug)]
enum Object {
    Asteroid,
    UndetectableAsteroid,
    AsteroidToBeLasered,
    Count(u32),
    SpaceStation,
    Empty,
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
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
                Object::AsteroidToBeLasered => '~',
                Object::SpaceStation => 'O',
                Object::Empty => '.',
                Object::Count(e ) => format!("{}", e).chars().next().unwrap(),
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

#[allow(dead_code)]
fn print_space(space: &Vec<Vec<Object>>, x_len: usize, y_len: usize) {
    for y in 0..y_len {
        for x in 0..x_len {
            print!("{}", space[y][x]);
        }
        println!();
    }
    println!();
}

#[allow(dead_code)]
fn print_space_with_counts(space: &mut Vec<Vec<Object>>, asteroids: &VecDeque<Point>, x_len: usize, y_len: usize) {
    let mut count = 1;
    for asteroid in asteroids {
        space[asteroid.y][asteroid.x] = Object::Count(count);
        count += 1;
    }
    for y in 0..y_len {
        for x in 0..x_len {
            print!("{}", space[y][x]);
        }
        println!();
    }
    println!();
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

fn mark_lasered_asteroid(space: &mut Vec<Vec<Object>>, station_pos: Point, pos: Point) -> Point{
    let mut curr_x_pos = pos.x as isize;
    let mut curr_y_pos = pos.y as isize;
    let x_diff = pos.x as isize - station_pos.x as isize;
    let y_diff = pos.y as isize - station_pos.y as isize;

    let mut position_to_be_lasered =  pos;

    let gcd = gcd((x_diff).abs(), (y_diff).abs());

    let x_diff= x_diff / gcd;
    let y_diff= y_diff / gcd;

    while curr_x_pos as usize != station_pos.x || curr_y_pos as usize != station_pos.y {
        let field = &mut space[curr_y_pos as usize][curr_x_pos as usize];
        if *field == Object::AsteroidToBeLasered {
            position_to_be_lasered = Point {x: curr_x_pos as usize, y: curr_y_pos as usize};
            break;
        }
        if *field == Object::Asteroid {
            position_to_be_lasered = Point {x: curr_x_pos as usize, y: curr_y_pos as usize}
        }
        curr_x_pos -= x_diff;
        curr_y_pos -= y_diff;
    }
    space[position_to_be_lasered.y][position_to_be_lasered.x] = Object::AsteroidToBeLasered;
    position_to_be_lasered
}

fn laser_iteration(
    mut space: &mut Vec<Vec<Object>>,
    station_pos: Point,
    x_len: usize,
    y_len: usize,
) -> VecDeque<Point>
{
    let mut asteroids_to_be_lasered_set: HashSet<Point> = HashSet::new();
    let mut asteroids_to_be_lasered: VecDeque<Point> = VecDeque::new();
    for y in 0..station_pos.y {
        let x = station_pos.x;
        if  x == 0 && station_pos.y == y {
            continue;
        };
        if space[y][x] == Object::Asteroid {
            let next_asteroid_to_be_lasered = mark_lasered_asteroid(
                &mut space,
                station_pos,
                Point { x: x, y: y },
            );
            asteroids_to_be_lasered_set.insert(next_asteroid_to_be_lasered);
        }
    }
    asteroids_to_be_lasered.append(&mut sort_asteroids(asteroids_to_be_lasered_set,station_pos, false));
    let mut asteroids_to_be_lasered_set: HashSet<Point> = HashSet::new();
    for y in 0..y_len {
        for x in (station_pos.x + 1)..x_len {
            if station_pos.x == x && station_pos.y == y {
                continue;
            };
            if space[y][x] == Object::Asteroid {
                let next_asteroid_to_be_lasered = mark_lasered_asteroid(
                    &mut space,
                    station_pos,
                    Point { x: x, y: y },
                );
                asteroids_to_be_lasered_set.insert(next_asteroid_to_be_lasered);
            }
        }
    }
    asteroids_to_be_lasered.append(&mut sort_asteroids(asteroids_to_be_lasered_set, station_pos, false));
    let mut asteroids_to_be_lasered_set: HashSet<Point> = HashSet::new();
    for y in (station_pos.y+1)..y_len {
        let x = station_pos.x;
        if  x == 0 && station_pos.y == y {
            continue;
        };
        if space[y][x] == Object::Asteroid {
            let next_asteroid_to_be_lasered = mark_lasered_asteroid(
                &mut space,
                station_pos,
                Point { x: x, y: y },
            );
            asteroids_to_be_lasered_set.insert(next_asteroid_to_be_lasered);
        }
    }
    asteroids_to_be_lasered.append(&mut sort_asteroids(asteroids_to_be_lasered_set, station_pos, false));
    let mut asteroids_to_be_lasered_set: HashSet<Point> = HashSet::new();
    for y in 0..y_len {
        for x in 0..station_pos.x {
            if space[y][x] == Object::Asteroid {
                let next_asteroid_to_be_lasered = mark_lasered_asteroid(
                    &mut space,
                    station_pos,
                    Point { x: x, y: y },
                );
                asteroids_to_be_lasered_set.insert(next_asteroid_to_be_lasered);
            }
        }
    }
    asteroids_to_be_lasered.append(&mut sort_asteroids(asteroids_to_be_lasered_set, station_pos, false));
    asteroids_to_be_lasered
}

#[derive(Eq, Debug)]
struct PointWithSlope {
    point: Point,
    slope: Rational,
}

impl Ord for PointWithSlope {
    fn cmp(&self, other: &Self) -> Ordering {
        self.slope.cmp(&other.slope)
    }
}

impl PartialOrd for PointWithSlope {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for PointWithSlope {
    fn eq(&self, other: &Self) -> bool {
        self.point == other.point
    }
}

fn sort_asteroids(asteroids: HashSet<Point>, station_pos: Point, negative: bool) -> VecDeque<Point> {
    let mut sorted_asteroids: VecDeque<Point> = VecDeque::new();
    let mut first = None;
    let mut last = None;
    let mut points_with_slopes = Vec::new();
    if !negative {
        for asteroid in asteroids {
            if asteroid.x as isize - station_pos.x as isize == 0 {
                if asteroid.y as isize - station_pos.y as isize > 0 {
                    first = Some(asteroid);
                } else {
                    last = Some(asteroid);
                }
                continue
            }
            let slope = Rational::new(asteroid.y as isize - station_pos.y as isize,
                                      asteroid.x as isize - station_pos.x as isize);
            points_with_slopes.push(PointWithSlope {point: asteroid, slope: slope});
        }
    }

    points_with_slopes.sort();
    for item in points_with_slopes {
        sorted_asteroids.push_back(item.point);
    }
    match first {
        Some(a) => sorted_asteroids.push_front(a),
        None => (),
    }
    match last {
        Some(a) => sorted_asteroids.push_back(a),
        None => (),
    }
    sorted_asteroids
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

fn get_ideal_number_of_detectable_asteroids(space: &Vec<Vec<Object>>, x_len: usize, y_len: usize) -> (usize, Point) {
    let mut max = 0;
    let mut pos = Point {x: 0, y: 0};
    for y in 0..y_len {
        for x in 0..x_len {
            if space[y][x] == Object::Asteroid {
                let result_space = mark_undetectable_asteroids(&space, Point { x: x, y: y }, x_len, y_len);
                let result= count_detectable_asteroids(&result_space, x_len, y_len);
                if result > max {
                    max = result;
                    pos.x = x;
                    pos.y = y;
                }
            }
        }
    }
    (max, pos)
}

fn part1() {
    let (space, x_len, y_len) = read_data("data");
    println!("Ideal Number: {}", get_ideal_number_of_detectable_asteroids(&space, x_len, y_len).0);
}

fn laser_asteroids(mut space: &mut Vec<Vec<Object>>, x_len: usize, y_len: usize) {
    let station_pos = get_ideal_number_of_detectable_asteroids(&space, x_len, y_len).1;
    space[station_pos.y][station_pos.x] = Object::SpaceStation;
    let mut asteroids_to_be_lasered = laser_iteration(&mut space, station_pos, x_len, y_len);

    for i in 1..201 {
        if asteroids_to_be_lasered.is_empty() {
            asteroids_to_be_lasered = laser_iteration(&mut space, station_pos, x_len, y_len);
        }
        let asteroid_to_laser = match asteroids_to_be_lasered.pop_front() {
            None => panic!("Not enough Asteroids."),
            Some(a) => a,
        };
        if i == 200 {
            println!("{}", asteroid_to_laser.x * 100 +  asteroid_to_laser.y);
        }
        space[asteroid_to_laser.y][asteroid_to_laser.x] = Object::Empty;
    }
}

fn part2() {
    let (mut space, x_len, y_len) = read_data("data");
    laser_asteroids(&mut space, x_len, y_len);
}

fn main() {
    part1();
    part2();
}
