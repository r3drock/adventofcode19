use std::fs;

fn read_data(path: &str) -> Vec<u8> {
    let mut digits = Vec::new();
    let data = fs::read_to_string(path)
        .expect("Something went wrong reading the file");
    for c in  data.chars() {
        digits.push(c.to_digit(10).unwrap() as u8);
    }
    digits
}

const PATTERN: [i8;4] = [0,1,0,-1];

fn get_index(repetitions: usize, i: usize) -> usize {
    let mut index = ((((i + 1) / repetitions ) % 4)) % 4;
    index
}
fn calculate_single_output(digits: &Vec<u8>, repetitions: usize) -> u8 {
    let mut sum: i64 = 0;
//    print!("{} ", repetitions);
    for (i, digit) in digits.iter().enumerate() {
        sum += *digit as i64 * PATTERN[get_index(repetitions, i)] as i64;
//        print!("{}*{} +", digit, PATTERN[get_index(repetitions, i)]);
    }
//    println!(" = {}", (sum % 10).abs());
    (sum % 10).abs() as u8
}

fn phase(digits: Vec<u8>) -> Vec<u8> {
    let mut newdigits = Vec::with_capacity(digits.len());

    for i in 0..digits.len() {
        newdigits.push(calculate_single_output(&digits, i + 1));
    }

    newdigits
}

fn main() {
    let mut digits = read_data("data");
    const ITERATIONS: usize = 100;
    for i in 0..ITERATIONS {
        digits = phase(digits);
    }
    for i in 0..8 {
        print!("{:?}", digits[i]);
    }
    println!();
}

