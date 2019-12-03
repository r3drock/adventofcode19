use std::fs;

fn calculate_fuel_needed(fuel : i64) -> i64 {
    let fuel = fuel / 3 - 2;
    if fuel > 0 {
        fuel + calculate_fuel_needed(fuel)
    } else { 0 }
}

fn main() {
    let data = fs::read_to_string("data")
        .expect("Something went wrong reading the file");
    let mut sum = 0;

    for line in data.split('\n') {
        sum += match line.parse::<i64>() {
            Ok(i) => calculate_fuel_needed(i),
            Err(_) => 0,
        };
    }
    println!("{}", sum);
}
