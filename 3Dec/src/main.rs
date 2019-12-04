use std::fs;
static mut x_start: isize = 0;
static mut y_start: isize = 0;
//fn parse_input(cable1: Vec<(&str,&str)>, cable2: Vec<(&str,&str)>) -> (Vec<(& str,& str)>,Vec<(& str,& str)>) {
//    let data = fs::read_to_string("data")
//        .expect("Something went wrong reading the file");
//    let mut lines = data.split('\n');
//    let cable1_directions:String = lines.next()
//        .expect("Line 1 is missing.").to_string();
//    let cable2_directions:String = lines.next()
//        .expect("Line 2 is missing.").to_string();
//    for direction in cable1_directions.split(',') {
//        cable1.push((&direction[..1], &direction[1..]));
//    }
//    for direction in cable2_directions.split(',') {
//        cable2.push((&direction[..1], &direction[1..]));
//    }
//    (cable1,cable2)
//}

fn calculate_dimensions (cable1: Vec<(char, isize)>, cable2: Vec<(char, isize)>) -> (isize, isize, isize, isize, isize, isize) {
    let mut y_min = 0;
    let mut y_max = 0;
    let mut x_min = 0;
    let mut x_max = 0;
    let mut x = 0;
    let mut y = 0;

    for (direction, length) in cable1 {
        match direction {
            'D' => {y -= length; if y < y_min {y_min = y}},
            'U' => {y += length; if y > y_max {y_max = y}},
            'L' => {x -= length; if x < x_min {x_min = x}},
            'R' => {x += length; if x > x_max {x_max = x}},
            _ => panic!("Wrong directions in input.")
        };
    }
    x = 0;
    y = 0;
    for (direction, length) in cable2 {
        match direction {
            'D' => {y -= length; if y < y_min {y_min = y}},
            'U' => {y += length; if y > y_max {y_max = y}},
            'L' => {x -= length; if x < x_min {x_min = x}},
            'R' => {x += length; if x > x_max {x_max = x}},
            _ => panic!("Wrong directions in input.")
        };
    }
    
    let y_len = (y_max - y_min + 1) as isize;
    let x_len = (x_max - x_min + 1) as isize;

    unsafe {
        x_start = -x_min as isize;
        y_start = -y_min as isize;
        println!("x_start: {}, y_start: {}, x_max: {}, y_max:{}, x_min: {}, y_min: {}, x_len: {}, y_len: {}",
                x_start, y_start, x_max, y_max, x_min, y_min, x_len, y_len);
    }

    (x_len, y_len, x_min, x_max, y_min, y_max)
}

fn set_at(x: isize, y: isize, x_len: isize, val: u8, field: &mut Vec<u8>) {
    unsafe {
        let index = (x + x_start + ((y + y_start ) * x_len)) as usize;
        field[index] = val;
    }
}

fn at(x: isize, y: isize, x_len: isize, field: &Vec<u8>) -> u8{
    unsafe {
        let index = (x + x_start + ((y + y_start ) * x_len)) as usize;
        field[index]
    }
}

fn printfield(x_min: isize, x_max: isize, y_min: isize, y_max: isize, x_len: isize, field: &Vec<u8>) {
    unsafe {let index = 0 + x_start + (((y_max-1) + y_start ) * x_len);
    println!("{}", index);
    }
    for y in (y_min..y_max+1).rev() {
        for x in x_min..x_max+1 {
            print!("{}", at(x, y, x_len, &field));
        }
        println!("");
    }
    //for (i, &item) in field.iter().enumerate() {
    //    print!("{}", item);
    //    if (i + 1) % x_len == 0 {println!("");}
    //}
}

fn main() {
    let mut cable1: Vec<(char, isize)> = Vec::new();
    let mut cable2: Vec<(char, isize)> = Vec::new();
    let data = fs::read_to_string("data")
        .expect("Something went wrong reading the file");
    let mut lines = data.split('\n');
    let cable1_moves:String = lines.next()
        .expect("Line 1 is missing.").to_string();
    let cable2_moves:String = lines.next()
        .expect("Line 2 is missing.").to_string();
    for move_specifier in cable1_moves.split(',') {
        let len_slice = &move_specifier[1..];
        let length: isize = len_slice.to_string().trim().parse().expect("No length present.");
        let direction = move_specifier.chars().next().expect("No direction present.");
        cable1.push((direction,length));
    }
    for move_specifier in cable2_moves.split(',') {
        let len_slice = &move_specifier[1..];
        let length: isize = len_slice.to_string().trim().parse().expect("No length present.");
        let direction = move_specifier.chars().next().expect("No direction present.");
        cable2.push((direction, length));
    }

    let (x_len, y_len, x_min, x_max, y_min, y_max) = calculate_dimensions(cable1, cable2);
    unsafe {
        println!("{} {} {} {}", x_len, y_len, x_start, y_start);
    }
    let mut field: Vec<u8> = vec![0;(x_len * y_len) as usize];
    set_at(0, 0, x_len, 4, &mut field);
    printfield(x_min, x_max, y_min, y_max, x_len, &field);
    unsafe{
    let index = 0 + x_start + ((0 + y_start ) * x_len);
    println!("index: {}", index)
    }

    //for (direction, length) in cable1 {
    //    println!("({}, {})", direction, length);
    //}
    //    println!("--------------------");
    //for (direction, length) in cable2 {
    //    println!("({}, {})", direction, length);
    //}
}
