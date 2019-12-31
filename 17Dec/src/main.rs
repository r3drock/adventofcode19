use intcomputer::intcode;
use intcomputer::intcode::Amplifier;

#[allow(dead_code)]
fn printfield(field: &Vec<Vec<u8>>) {
    let y_len = field.len();
    let x_len = field[0].len();
    for y in 0..(y_len) {
        for x in 0..x_len {
            print!("{}", match field[y].get(x) {
                Some(a) => *a as char,
                None => break,
            } );
        }
        println!();
    }
}

#[derive(Copy, Clone, Debug)]
struct Point {
    x: usize,
    y: usize,
}

fn find_intersections(field: &Vec<Vec<u8>>) -> Vec<Point> {
    let y_len = field.len();
    let x_len = field[0].len();

    let mut intersections = Vec::new();

    for y in 1..(y_len-1) {
        for x in 1..(x_len-1) {
            if  field[y][x] == '#' as u8 &&
                field[y][x + 1] == '#' as u8 &&
                field[y][x - 1] == '#' as u8 &&
                field[y + 1][x] == '#' as u8 &&
                field[y - 1][x] == '#' as u8 {
                intersections.push(Point {x, y});
            }
        }
    }
    intersections
}

fn calculate_sum_of_alignment_parameters(intersections: Vec<Point>) -> usize {
    let mut sum = 0;
    for intersection in intersections {
        sum += intersection.x * intersection.y;
    }
    sum
}

fn read_field(mut computer: Amplifier) -> Vec<Vec<u8>> {
    let mut field = vec![vec![0;0];0];
    assert_eq!(0, field.len());
    field.push(vec![]);
    assert_eq!(1, field.len());
    let mut y = 0;
    while let Some(c) = computer.run_program_until_output(false) {
        match (c as u8) as char {
            '\n' => {
                y += 1;
                field.push(vec![])
            },
            _ => field[y].push(c as u8),
        }
    }
    field.remove(y);
    field.remove(y - 1);
    field
}

fn main() {
    let program = intcode::read_data("program");
    let computer = intcode::Amplifier::new(program, vec![]);
    let field = read_field(computer);

//    printfield(&field);
    let intersections = find_intersections(&field);
    let sum = calculate_sum_of_alignment_parameters(intersections);
    println!("Sum: {}", sum);

}
