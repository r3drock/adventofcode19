use intcomputer::intcode;
use intcomputer::intcode::Amplifier;
use std::collections::{HashMap, VecDeque};
use petgraph::graphmap::UnGraphMap;
use petgraph::dot::{Config, Dot};
use petgraph::algo::astar;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone, Ord, PartialOrd)]
struct Point {
    x: isize,
    y: isize,
}

fn recursion(
    computer: &mut Amplifier,
    mut field: &mut HashMap<Point, char>,
    pos: Point,
    steps: usize,
    visited: HashMap<Point,()>,
) -> Option<Point> {
    let mut target_pos = None;

    for i in 1..5 {
        match walk_around(computer.clone(),
            &mut field, pos, steps, visited.clone(), i) {
            Some(a) => target_pos = Some(a),
            None => (),
        }
    }

    target_pos
}

fn walk_around(
    mut computer: Amplifier,
    mut field: &mut HashMap<Point, char>,
    mut pos: Point,
    steps: usize,
    mut visited: HashMap<Point,()>,
    direction: isize
) -> Option<Point> {
    let mut target_pos = None;

    let move_pos = match direction {
        1 => Point {x: pos.x, y: pos.y - 1}, //north
        2 => Point {x: pos.x, y: pos.y + 1}, //south
        3 => Point {x: pos.x - 1, y: pos.y}, //west
        4 => Point {x: pos.x + 1, y: pos.y}, //east
        _ => panic!("wrong direction"),
    };
    if visited.contains_key(&move_pos) {
       return target_pos;
    } else {
        visited.insert(move_pos, ());
    }

    computer.push_input(direction);
    target_pos =
    match computer.run_program_until_output(false).unwrap() {
        0 => {
            field.insert(move_pos, '#');
            None
        },
        1 => {
            pos = move_pos;
            field.insert(pos, '.');
            recursion(&mut computer, &mut field, pos, steps, visited)
        },
        2 => {
            pos = move_pos;
            field.insert(pos , 'O');
            recursion(&mut computer, &mut field, move_pos, steps, visited);
            Some(pos)
        },
        _ => panic!("invalid output!"),
    };
    target_pos
}

fn get_dimensions(field: &HashMap<Point, char>) -> (isize, isize, isize, isize) {
    let mut x_min = 0;
    let mut y_min = 0;
    let mut x_max = 0;
    let mut y_max = 0;
    for (point, _) in field {
        if point.x < x_min {
            x_min = point.x;
        }
        if point.x > x_max {
            x_max = point.x;
        }
        if point.y < y_min {
            y_min = point.y;
        }
        if point.y > y_max {
            y_max = point.y;
        }
    }
    (x_min, y_min, x_max, y_max)
}

#[allow(dead_code)]
fn print_field(field: &HashMap<Point, char>) {
    let (x_min, y_min, x_max, y_max) = get_dimensions(&field);

    println!("y: {} until {}", y_min, y_max + 1);
    println!("x: {} until {}", x_min, x_max + 1);

    for y in y_min..(y_max+1) {
        for x in x_min..(x_max+1) {
            print!("{}", field.get(&Point { x, y }).unwrap_or_else(|| &' '));
        }
        println!();
    }
}

fn add_neighbours(field: &HashMap<Point, char>, graph: &mut UnGraphMap<Point, u8>, point1: Point, point2: Point) {
    let neighbour = field.get(&point2).unwrap_or_else(|| &' ');
    if *neighbour == '.' || *neighbour == 'O' {
        graph.add_edge(point1, point2, 1);
    }
}

fn convert_to_graph(field: &HashMap<Point, char>) -> UnGraphMap<Point, u8> {
 let mut graph = UnGraphMap::<Point, u8>::new();

 let (x_min, y_min, x_max, y_max) = get_dimensions(&field);
    for y in (y_min+1)..y_max {
        for x in (x_min+1)..x_max {
            let point1 = Point { x, y };
            let value = field.get(&point1).unwrap_or_else(|| &' ');
            if '.' == *value || 'O' == *value {
                add_neighbours(&field, &mut graph, point1, Point { x: x + 1, y: y});
                add_neighbours(&field, &mut graph, point1, Point { x: x - 1, y: y});
                add_neighbours(&field, &mut graph, point1, Point { x: x, y: y + 1});
                add_neighbours(&field, &mut graph, point1, Point { x: x, y: y - 1});
            }
        }
    }
 graph
}

fn fill_with_oxygen(graph: &UnGraphMap<Point, u8>, pos: Point, visited: &mut HashMap<Point, usize>) -> usize {
    let mut minutes = 0;
    let mut queue: VecDeque<(Point, usize)> = VecDeque::new();
    queue.push_back((pos, minutes));
    visited.insert(pos, minutes);
    while !queue.is_empty() {
        let (new_pos, mins) = queue.pop_front().unwrap();
        minutes = mins;
        minutes += 1;
        for neighbour in graph.neighbors(new_pos) {
            if !visited.contains_key(&neighbour) {
                visited.insert(neighbour, minutes);
                queue.push_back((neighbour, minutes));
            }
        }
    }
    minutes - 1
}

fn main() {
    let program = intcode::read_data("program");
    let mut computer = intcode::Amplifier::new(program, vec![]);

    let mut field: HashMap<Point, char> = HashMap::new();
    field.insert(Point { x: 0, y: 0 }, '.');

    let mut visited = HashMap::new();
    visited.insert(Point { x: 0, y: 0 }, ());
    let target_pos = recursion(&mut computer, &mut field, Point { x: 0, y: 0 }, 0, visited).unwrap();

    let graph = convert_to_graph(&field);
    let len = astar(&graph, Point {x: 0, y: 0}, |finish| finish == target_pos, |_| 1, |_| 0).unwrap().0;

    println!("{}", len);
    let mut visited = HashMap::new();
    println!("{}", fill_with_oxygen(&graph, target_pos, &mut visited));
//    println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));
//    print_field(&field);
}

