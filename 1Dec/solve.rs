use std::fs;

fn main() {
    let data = fs::read_to_string("data")
        .expect("Something went wrong reading the file");
    let masses : Vec<i64> = data.split_whitespace().map(|s| s.parse::<i64>().unwrap()).collect();
    let fuel_requirements : Vec<i64> = masses.into_iter().map(|x| x/3 - 2).collect();
    let sum : i64 = fuel_requirements.into_iter().sum();
    println!("{}", sum);
}
