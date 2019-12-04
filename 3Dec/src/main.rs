fn test_password(mut value: u32) -> u32 {
    let mut adjacency = false;

    let mut prev = 10;
    //go from right to left digit
    while value > 0 {
        let current_digit = value % 10;
        if prev == current_digit {
            adjacency = true;
        }
        if prev < current_digit {
            return 0;
        }
        prev = current_digit;
        value /= 10;
    }
    if adjacency {1} else {0}
}

fn main() {
    let mut count = 0;
    for value in 245182..790572 {
        count += test_password(value);
    }
    println!("{}", count);
}
