use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
struct Reaction {
    inputs: Vec<(usize, String)>,
    output: (usize, String),
}

fn parse_chemical(input: &str, linecount: usize) -> (usize, String) {
    let mut iter = input.trim().split(" ");
    let amount = match iter.next() {
        Some(x) => match x.parse::<usize>() {
            Ok(y) => y,
            Err(_) => panic!("Line: {} not a number", linecount),
        },
        None => panic!("expected amount"),
    };
    let chemical = match iter.next() {
        Some(x) => x.to_string(),
        None => panic!("expected amount"),
    };
    (amount, chemical)
}

fn read_data(file_name: &str) -> Vec<Reaction> {
    let mut reactions = Vec::new();
    let data: String =
        fs::read_to_string(file_name).expect("Something went wrong reading the file");

    let mut linecount = 0;
    for line in data.split('\n') {
        linecount += 1;
        let mut reaction_inputs: Vec<(usize, String)> = Vec::new();
        let mut chemicals = line.split("=>");
        let inputs = chemicals.next();
        for input in inputs.unwrap().split(',') {
            reaction_inputs.push(parse_chemical(input, linecount));
        }
        let output = chemicals.next().expect("No Output");
        reactions.push(Reaction {
            inputs: reaction_inputs,
            output: parse_chemical(output, linecount),
        });
    }
    reactions
}

fn get_costs(
    reaction_refs: &HashMap<&str, (&Vec<(usize, String)>, usize)>,
    overproduced: &mut HashMap<&str, usize>,
    output: (usize, &str),
) -> usize {
    let mut costs = 0;
    let mut output_amount = output.0;
    let output_chemical = output.1;
    if output_chemical == "ORE" {
        return output_amount;
    }

    if output_amount <= overproduced[output_chemical] {
        *overproduced.get_mut(output_chemical).unwrap() -= output_amount;
        return 0;
    }
    if overproduced[output_chemical] > 0 {
        output_amount -= overproduced[output_chemical];
        *overproduced.get_mut(output_chemical).unwrap() = 0;
    }

    let amount_produced_per_run = reaction_refs[output_chemical].1;
    let production_runs = (output_amount + (amount_produced_per_run - 1)) / amount_produced_per_run;
    *overproduced.get_mut(output_chemical).unwrap() +=
        (amount_produced_per_run * production_runs) - output_amount;
    for _ in 0..production_runs {
        for input in reaction_refs[output_chemical].0 {
            let amount_needed = input.0;
            let chemical = &input.1[..];
            costs += get_costs(reaction_refs, overproduced, (amount_needed, chemical));
        }
    }
    costs
}

fn main() {
    let reactions = read_data("data");
    let mut reaction_refs: HashMap<&str, (&Vec<(usize, String)>, usize)> = HashMap::new();
    let mut overproduced: HashMap<&str, usize> = HashMap::new();

    for reaction in &reactions {
        let output = &reaction.output.1;
        reaction_refs.insert(output, (&reaction.inputs, reaction.output.0));
        overproduced.insert(output, 0);
    }

    let mut costs = get_costs(&reaction_refs, &mut overproduced, (1, "FUEL"));
    println!("{}", costs);
    let mut max_amount_of_fuel: usize = 0;
    let mut fuel_left: usize = 1000000000000;

    while fuel_left >= costs {
        max_amount_of_fuel += 1;
        fuel_left -= costs;
        costs = get_costs(&reaction_refs, &mut overproduced, (1, "FUEL"));
    }
    println!("{}", max_amount_of_fuel);
}
