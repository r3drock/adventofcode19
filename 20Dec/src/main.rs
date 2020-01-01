use petgraph::graphmap;
use std::fs;
use std::collections::HashMap;
#[allow(unused_imports)]
use petgraph::dot::{Config, Dot};


#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone, Ord, PartialOrd)]
struct Point {
    x: usize,
    y: usize,
}

fn calculate_thickness(char_array: &Vec<Vec<char>>, x_len: usize, y_len: usize) -> usize {
    for x in 2..x_len {
        if char_array[y_len / 2][x] == ' ' || char_array[y_len / 2][x].is_ascii_alphabetic() {
            return x - 2;
        }
    }
    panic!("Thickness can't be calculated!");
}

fn get_outside_portals(char_array: &Vec<Vec<char>>, x_len: usize, y_len: usize) -> HashMap<String, Point> {
    let mut portals: HashMap<String, Point> = HashMap::new();
    for x in (2)..(x_len - 2) {
        if char_array[0][x].is_ascii_alphabetic() && char_array[1][x].is_ascii_alphabetic() {
            portals.insert(format!("{}{}", char_array[0][x], char_array[1][x]), Point { x: x, y: 2 });
        }
        if char_array[y_len - 2][x].is_ascii_alphabetic() && char_array[y_len - 1][x].is_ascii_alphabetic() {
            portals.insert(format!("{}{}", char_array[y_len - 2][x], char_array[y_len - 1][x]), Point { x: x, y: y_len - 3 });
        }
    }
    for y in (2)..(y_len - 2) {
        if char_array[y][0].is_ascii_alphabetic() && char_array[y][1].is_ascii_alphabetic() {
            portals.insert(format!("{}{}", char_array[y][0], char_array[y][1]), Point { x: 2, y: y });
        }
        if char_array[y][x_len - 2].is_ascii_alphabetic() && char_array[y][x_len - 1].is_ascii_alphabetic() {
            portals.insert(format!("{}{}", char_array[y][x_len - 2], char_array[y][x_len - 1]), Point { x: x_len - 3, y: y });
        }
    }
    portals
}

fn connect_normal_paths(char_array: &Vec<Vec<char>>, x_len: usize, y_len: usize) -> graphmap::UnGraphMap<Point, usize> {
    let mut graph = graphmap::UnGraphMap::new();
    for y in 1..y_len {
        for x in 1..x_len {
            if char_array[y][x] == '.' {
                graph.add_node(Point { x, y });
                if char_array[y - 1][x] == '.' {
                    graph.add_edge(Point { x, y }, Point { x: x, y: y - 1 }, 1);
                }
                if char_array[y][x - 1] == '.' {
                    graph.add_edge(Point { x, y }, Point { x: x - 1, y: y }, 1);
                }
            }
        }
    }
    graph
}

fn connect_portals(char_array: &Vec<Vec<char>>, portals: & HashMap<String, Point>, graph: &mut graphmap::UnGraphMap<Point, usize>, x_len: usize, y_len: usize, thickness: usize) {
    for x in (2 + thickness)..(x_len - 2 - thickness) {
        if char_array[2 + thickness][x].is_ascii_alphabetic() && char_array[2 + thickness + 1][x].is_ascii_alphabetic() {
            let portal_end = portals[&format!("{}{}", char_array[2 + thickness][x], char_array[2 + thickness + 1][x])];
            graph.add_edge(portal_end, Point { x: x, y: 2 + thickness - 1 }, 1);
        }
        if char_array[y_len - thickness - 3 - 1][x].is_ascii_alphabetic() && char_array[y_len - thickness - 2 - 1][x].is_ascii_alphabetic() {
            let portal_end = portals[&format!("{}{}", char_array[y_len - thickness - 3 - 1][x], char_array[y_len - thickness - 2 - 1][x])];
            graph.add_edge(portal_end, Point { x: x, y: y_len - thickness - 2 }, 1);
        }
    }
    for y in (2 + thickness)..(y_len - 2 - thickness) {
        if char_array[y][2 + thickness].is_ascii_alphabetic() && char_array[y][2 + thickness + 1].is_ascii_alphabetic() {
            let portal_end = portals[&format!("{}{}", char_array[y][2 + thickness], char_array[y][2 + thickness + 1])];
            graph.add_edge(portal_end, Point { x: 2 + thickness - 1, y: y }, 1);
        }
        if char_array[y][x_len - 3 - thickness - 1].is_ascii_alphabetic() && char_array[y][x_len - thickness - 2 - 1].is_ascii_alphabetic() {
            let portal_end = portals[&format!("{}{}", char_array[y][x_len - 3 - thickness - 1], char_array[y][x_len - 2 - thickness - 1])];
            graph.add_edge(portal_end, Point { x: x_len - thickness - 2, y: y }, 1);
        }
    }
}

fn read_char_array(path: &str) -> Vec<Vec<char>> {
    let data = fs::read_to_string(path)
        .expect("Something went wrong reading the file");

    let mut char_array: Vec<Vec<char>> = Vec::new();

    let mut y = 0;
    for line in data.split('\n') {
        char_array.push(vec![]);
        for c in line.chars() {
            char_array[y].push(c);
        }
        y += 1;
    }
    char_array.remove(char_array.len() - 1);
    char_array
}

fn build_maze_graph(path: &str) -> (graphmap::UnGraphMap<Point, usize>, Point, Point) {
    let char_array = read_char_array(path);

    let y_len = char_array.len();
    let x_len = char_array[0].len();
    let thickness = calculate_thickness(&char_array, x_len, y_len);

    let mut graph = connect_normal_paths(&char_array, x_len, y_len);

    let portals = get_outside_portals(&char_array, x_len, y_len);

    connect_portals(&char_array, &portals, &mut graph, x_len, y_len, thickness);

    let start_point = portals["AA"];
    let end_point = portals["ZZ"];

//    println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));

    (graph, start_point, end_point)
}

fn main() {
    let (maze_graph, start_point, end_point) = build_maze_graph("maze");
    let len = petgraph::algo::astar(&maze_graph, start_point, |finish| finish == end_point, |_| 1, |_| 0).unwrap().0;
    println!("{}", len);
}
