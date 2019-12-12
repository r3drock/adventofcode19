use std::fs;
use petgraph::Graph;
use petgraph::visit::Topo;
use petgraph::dot::{Dot,Config};

fn add_edges_to_tree(graph: &mut Graph<String, u8>, orbit_description: &str) {
    let mut desc = orbit_description.split(')');
    let planet: String = String::from(desc.next().expect("No planet object found."));
    let orbiter: String = String::from(desc.next().expect("No orbiter object found."));

    let mut topo = Topo::new(&*graph);
    'outer: while let Some(nx) = topo.next(&*graph) {
        if graph[nx] == planet {
            let mut topo = Topo::new(&*graph);
            while let Some(ny) = topo.next(&*graph) {
                if graph[ny] == orbiter {
                    if !graph.contains_edge(nx, ny) {
                        graph.add_edge(nx, ny, 1);
                    }
                    break 'outer;
                }
            }
            break;
        }
    }
}

fn add_nodes_to_tree(graph: &mut Graph<String, u8>, orbit_description: &str) {
    let mut desc = orbit_description.split(')');
    let planet: String = String::from(desc.next().expect("No planet object found."));
    let orbiter: String = String::from(desc.next().expect("No orbiter object found."));
    let mut insert_planet = true;
    let mut insert_orbiter = true;

    let mut topo = Topo::new(&*graph);
    while let Some(nx) = topo.next(&*graph) {
        if graph[nx] == planet {
            insert_planet = false
        }
        if graph[nx] == orbiter {
            insert_orbiter = false
        }
        if !insert_planet && !insert_orbiter {break}
    }
    if insert_planet{
        let n = graph.add_node(planet);
    }
    if insert_orbiter{
        let n = graph.add_node(orbiter);
    }
}


fn count_number_of_orbits (graph: &Graph<String, u8>, root: &petgraph::graph::NodeIndex, mut depth: usize) -> usize {
    let mut number_of_orbits: usize = depth;
    depth += 1;
    let mut neighbours = graph.neighbors_directed(*root, petgraph::Direction::Outgoing);

    while let Some(nx) = neighbours.next() {
        number_of_orbits += count_number_of_orbits(&graph, &nx, depth);
    }

    number_of_orbits
}

fn main() {
    let data = fs::read_to_string("data")
        .expect("Something went wrong reading the file");
    let mut graph = Graph::<String, u8>::new();
    let root: petgraph::graph::NodeIndex = graph.add_node(String::from("COM"));

    for line in data.split('\n') {
        add_nodes_to_tree(&mut graph, &line);
        add_edges_to_tree(&mut graph, &line);
    }
    println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));
    println!("{}", count_number_of_orbits(&graph, &root, 0));
}
