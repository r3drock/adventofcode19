use std::fs;

fn main() {
    let data = fs::read_to_string("data")
        .expect("Something went wrong reading the file");
    let lines = data.split('\n');
    let mut sum = 0;
    for line in lines {
        sum += match line.parse::<i32>() {
            Ok(i) => i/3-2,
            Err(_) => 0,
        };
    }
    println!("{}", sum);
}
