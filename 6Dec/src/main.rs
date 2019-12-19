use std::fs;
use petgraph::graphmap::{DiGraphMap,UnGraphMap};
use petgraph::dot::{Dot,Config};


fn add_orbit_to_undirected_graph<'a>(mut graph: UnGraphMap<&'a str, u8>, orbit_description: &'a str) -> UnGraphMap<&'a str, u8> {
    let mut desc = orbit_description.split(')');
    let planet = desc.next().expect("No planet object found.");
    let orbiter = desc.next().expect("No orbiter object found.");

    if ! graph.contains_node(planet){
        graph.add_node(planet);
    }
    if ! graph.contains_node(orbiter){
        graph.add_node(orbiter);
    }
    if ! graph.contains_edge(planet, orbiter) {
        graph.add_edge(planet, orbiter, 1);
    }

    graph
}

fn add_orbit_to_graph<'a>(mut graph: DiGraphMap<&'a str, u8>, orbit_description: &'a str) -> DiGraphMap<&'a str, u8> {
    let mut desc = orbit_description.split(')');
    let planet = desc.next().expect("No planet object found.");
    let orbiter = desc.next().expect("No orbiter object found.");

    if ! graph.contains_node(planet){
        graph.add_node(planet);
    }
    if ! graph.contains_node(orbiter){
        graph.add_node(orbiter);
    }
    if ! graph.contains_edge(planet, orbiter) {
        graph.add_edge(planet, orbiter, 1);
    }

    graph
}


fn count_number_of_orbits (graph: &DiGraphMap<&str, u8>, root: &str, mut depth: usize) -> usize {
    let mut number_of_orbits: usize = depth;
    depth += 1;
    let mut neighbours = graph.neighbors_directed(root,petgraph::Direction::Outgoing);

    while let Some(nx) = neighbours.next() {
        number_of_orbits += count_number_of_orbits(&graph, &nx, depth);
    }
    number_of_orbits
}

fn main() {
    let data = fs::read_to_string("data")
        .expect("Something went wrong reading the file");
    let mut digraph = DiGraphMap::<&str, u8>::new();
    let mut undigraph = UnGraphMap::<&str, u8>::new();
    let root = digraph.add_node("COM");
    let un_root = undigraph.add_node("COM");

    for line in data.split('\n') {
        digraph = add_orbit_to_graph(digraph, &line);
        undigraph = add_orbit_to_undirected_graph(undigraph, &line);
    }

    println!("{:?}", Dot::with_config(&undigraph, &[Config::EdgeNoLabel]));
    println!("{}", count_number_of_orbits(&digraph, &root, 0));
}
