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
fn test_password2(mut value: u32) -> u32 {
    let mut adjacency = false;
    let mut impossible_digit = 10;
    let mut prevprev = 10;
    let mut prev = value % 10;
    value /= 10;
    //go from right to left digit
    while prev > 0 {
        let current_digit = value % 10;
        if current_digit == prev && prev == prevprev {
            impossible_digit = current_digit;
        } else if prev == prevprev && current_digit != prev && impossible_digit != prev {
            adjacency = true;
        }
        prevprev = prev;
        prev = current_digit;
        value /= 10;
    }
    if adjacency {1} else {0}
}

fn main() {
    let mut count1 = 0;
    let mut count2 = 0;
    for value in 245182..790572 {
        if (test_password(value)) == 1 {
            count1 += 1;
            count2 += test_password2(value);
        }
    }
    println!("part1: {}", count1);
    println!("part2: {}", count2);
}
