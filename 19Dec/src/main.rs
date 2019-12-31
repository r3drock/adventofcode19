use intcomputer::intcode;

const X_LEN: usize = 100;
const Y_LEN: usize = 100;


fn map_area(mut computer: &intcode::Amplifier) -> [[bool;Y_LEN];X_LEN] {

    let mut area = [[false; Y_LEN]; X_LEN];
    for y in 0..Y_LEN {
        for x in 0..X_LEN {
            let mut computer = computer.clone();
            computer.push_input(x as isize);
            computer.push_input(y as isize);
            area[x][y] = if 1 == computer.run_program_until_output(false).unwrap() {true} else {false};
        }
    }
    area
}

fn print_area(area: &[[bool;Y_LEN];X_LEN]) {
    for y in 0..Y_LEN {
        for x in 0..X_LEN {
            print!("{}",if area[x][y] {'1'} else {'0'});
        }
        println!();
    }
}

fn count_affected_points(area: &[[bool;Y_LEN];X_LEN]) -> usize {
    let mut count = 0;
    for y in 0..Y_LEN {
        for x in 0..X_LEN {
            count += if area[x][y] {1} else {0};
        }
    }
    count
}

fn main() {
    let program = intcode::read_data("program");
    let computer = intcode::Amplifier::new(program, vec![]);
    let area = map_area(&computer);
    print_area(&area);
    println!("{}", count_affected_points(&area));
}
