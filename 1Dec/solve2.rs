use std::fs;

fn calculate_fuel_needed(a : i64) -> i64 {
    let b = a / 3 - 2;
    return match b {
        std::i64::MIN ..= 0 => 0,
        b  => b + calculate_fuel_needed(b),
    };
}

fn main() {
    let data = fs::read_to_string("data")
        .expect("Something went wrong reading the file");
    let lines = data.split('\n');
    let mut sum = 0;
    for line in lines {
        sum += match line.parse::<i64>() {
            Ok(i) => calculate_fuel_needed(i),
            Err(_) => 0,
        };
    }
    println!("{}", sum);
}
