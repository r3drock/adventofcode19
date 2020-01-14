extern crate ansi_term;

use ansi_term::Colour::*;
use std::fs;

static mut X_START: isize = 0;
static mut Y_START: isize = 0;

fn calculate_dimensions (cable1: &Vec<(char, isize)>, cable2: &Vec<(char, isize)>)
-> (isize, isize, isize, isize, isize, isize) {
    let mut y_min = 0;
    let mut y_max = 0;
    let mut x_min = 0;
    let mut x_max = 0;
    let mut x = 0;
    let mut y = 0;

    for (direction, length) in cable1 {
        match direction {
            'D' => {y -= *length; if y < y_min {y_min = y}},
            'U' => {y += *length; if y > y_max {y_max = y}},
            'L' => {x -= *length; if x < x_min {x_min = x}},
            'R' => {x += *length; if x > x_max {x_max = x}},
            _ => panic!("Wrong directions in input.")
        };
    }
    x = 0;
    y = 0;
    for (direction, length) in cable2 {
        match direction {
            'D' => {y -= *length; if y < y_min {y_min = y}},
            'U' => {y += *length; if y > y_max {y_max = y}},
            'L' => {x -= *length; if x < x_min {x_min = x}},
            'R' => {x += *length; if x > x_max {x_max = x}},
            _ => panic!("Wrong directions in input.")
        };
    }
    
    let y_len = (y_max - y_min + 1) as isize;
    let x_len = (x_max - x_min + 1) as isize;

    unsafe {
        X_START = -x_min as isize;
        Y_START = -y_min as isize;
    }

    (x_len, y_len, x_min, x_max, y_min, y_max)
}

fn set_at_u32(x: isize, y: isize, x_len: isize, val: u32, field: &mut Vec<u32>) {
    unsafe {
        let index = (x + X_START + ((y + Y_START ) * x_len)) as usize;
        field[index] = val;
    }
}

fn at_u32(x: isize, y: isize, x_len: isize, field: &Vec<u32>) -> u32{
    unsafe {
        let index = (x + X_START + ((y + Y_START ) * x_len)) as usize;
        field[index]
    }
}
fn set_at(x: isize, y: isize, x_len: isize, val: u8, field: &mut Vec<u8>) {
    unsafe {
        let index = (x + X_START + ((y + Y_START ) * x_len)) as usize;
        field[index] = val;
    }
}

fn at(x: isize, y: isize, x_len: isize, field: &Vec<u8>) -> u8{
    unsafe {
        let index = (x + X_START + ((y + Y_START ) * x_len)) as usize;
        field[index]
    }
}

fn printfield(x_min: isize, x_max: isize, y_min: isize, y_max: isize,
              x_len: isize, field: &Vec<u8>) {
    for _ in 0..x_len {print!("█");}
    println!();

    for y in (y_min..y_max+1).rev() {
        for x in x_min..x_max+1 {
            let color = at(x, y, x_len, &field);
            match color {
                0 => print!("{}", 0),
                1 => print!("{}", Green.bold().paint("1")),
                2 => print!("{}", Blue.bold().paint("2")),
                3 => print!("{}", Yellow.on(Black).bold().paint("3")),
                4 => print!("{}", Red.bold().paint("4")),
                x => print!("{}", x),
            }
        }
        println!("");
    }

    for _ in 0..x_len {print!("█");}
    println!();
}

#[inline]
fn get_new_color(current_color: u8, color: u8) -> u8 {
    if current_color == color {
        color
    } else if current_color == 0 &&  current_color != 4 {
        color
    } else {
        3
    }
}

fn lay_cable (cable: Vec<(char, isize)>, x_len: isize, color: u8,
              mut field: &mut Vec<u8>, mut distancefield_cable: &mut Vec<u32>) {
    let mut x = 0;
    let mut y = 0;

    let mut current_length = 0;
    for (direction, length) in cable {
        match direction {
            'D' => {
                for _ in 0..length {
                    current_length += 1;
                    y -= 1;
                    let current_color = at(x, y, x_len, &field);
                    set_at(x, y, x_len, get_new_color(current_color, color), &mut field);
                    set_at_u32(x, y, x_len, current_length, &mut distancefield_cable);
                }
            },
            'U' => {
                for _ in 0..length {
                    current_length += 1;
                    y += 1;
                    let current_color = at(x, y, x_len, &field);
                    set_at(x, y, x_len, get_new_color(current_color, color), &mut field);
                    set_at_u32(x, y, x_len, current_length, &mut distancefield_cable);
                }
            },
            'L' => {
                for _ in 0..length {
                    current_length += 1;
                    x -= 1;
                    let current_color = at(x, y, x_len, &field);
                    set_at(x, y, x_len, get_new_color(current_color, color), &mut field);
                    set_at_u32(x, y, x_len, current_length, &mut distancefield_cable);
                }
            },
            'R' => {
                for _ in 0..length {
                    current_length += 1;
                    x += 1;
                    let current_color = at(x, y, x_len, &field);
                    set_at(x, y, x_len, get_new_color(current_color, color), &mut field);
                    set_at_u32(x, y, x_len, current_length, &mut distancefield_cable);
                }
            },
            _ => panic!("Wrong directions in input.")
        };
    }
}

fn calculate_min_distance(x_min: isize, x_max: isize, y_min: isize,
                                    y_max: isize, x_len: isize,
                                    field: &Vec<u8>, distancefield_cable1: &Vec<u32>,
                                    distancefield_cable2: &Vec<u32>) -> u32 {
    let mut min_distance = std::u32::MAX;
    for y in (y_min..y_max+1).rev() {
        for x in x_min..x_max+1 {
            if at(x,y, x_len, &field) == 3 &&
                  min_distance > (at_u32(x, y, x_len, &distancefield_cable1) +
                  at_u32(x, y, x_len, &distancefield_cable2))
            {
                min_distance = at_u32(x, y, x_len, &distancefield_cable1) +
                    at_u32(x, y, x_len, &distancefield_cable2);
            };
        }
    }
    min_distance
}

fn calculate_min_manhattan_distance(x_min: isize, x_max: isize, y_min: isize,
                          y_max: isize, x_len: isize,
                          field: &Vec<u8>) -> u32 {
    let mut min_distance = std::isize::MAX;
    for y in (y_min..y_max+1).rev() {
        for x in x_min..x_max+1 {
            if at(x,y, x_len, &field) == 3 {
                if (x.abs() + y.abs()) < min_distance{
                    min_distance = x.abs() + y.abs();
                }
            };
        }
    }
    min_distance as u32
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

    let (x_len, y_len, x_min, x_max, y_min, y_max) = calculate_dimensions(&cable1, &cable2);
    println!("field_dimensions: ({} x {})", x_len, y_len);

    let mut field: Vec<u8> = vec![0;(x_len * y_len) as usize];

    //set startingpoint
    set_at(0, 0, x_len, 4, &mut field);

    let mut distancefield_cable1: Vec<u32> = vec![0;(x_len * y_len) as usize];
    let mut distancefield_cable2: Vec<u32> = vec![0;(x_len * y_len) as usize];
    lay_cable(cable1, x_len, 1, &mut field, &mut distancefield_cable1);//, &mut distancefield);
    lay_cable(cable2, x_len, 2, &mut field, &mut distancefield_cable2);

    if x_len < 200 && y_len < 200 {
        printfield(x_min, x_max, y_min, y_max, x_len, &field);
    };

    println!("part1: min manhattan distance: {}",
             calculate_min_manhattan_distance(x_min, x_max, y_min, y_max, x_len, &field));

    println!("part2: min distance: {}",
             calculate_min_distance(x_min, x_max, y_min, y_max, x_len, &field,
                                    &distancefield_cable1, &distancefield_cable2));
}