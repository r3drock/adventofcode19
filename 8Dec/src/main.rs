use std::fs;

const X_LEN: usize = 25;
const Y_LEN: usize = 6;

fn read_data() -> Vec<u8> {
    let mut digits: Vec<u8> = Vec::new();
    let data = fs::read_to_string("data")
        .expect("Something went wrong reading the file");
    for digit in data.chars() {
        digits.push(digit.to_digit(10).unwrap() as u8);
    }
    digits
}

fn at(digits: &Vec<u8>, i: usize, y: usize , x: usize) -> u8 {
    digits[(i*X_LEN*Y_LEN) + (y*X_LEN) + x]
}

fn get_layers(digits: Vec<u8>) -> (Vec<Vec<Vec<u8>>>, usize) {
    let len = digits.len();
    let layer_count = len / (X_LEN * Y_LEN);
    let mut layer_with_fewest_zeros_index = 0;
    let mut fewest_zeros = std::usize::MAX;

    let mut layers = vec![vec![vec![0;X_LEN];Y_LEN];layer_count];
    for i in 0..layer_count {
        let mut current_layer_zero_count = 0;
        for y in 0..Y_LEN {
            for x in 0..X_LEN {
                let temp = at(&digits, i, y, x);
                current_layer_zero_count += {if temp == 0 {1} else {0}};
                layers[i][y][x] = temp;
            }
        }
        if current_layer_zero_count < fewest_zeros {
            layer_with_fewest_zeros_index = i;
            fewest_zeros = current_layer_zero_count;
        } 
    }
    (layers, layer_with_fewest_zeros_index)
}

fn get_magic_number(layer: Vec<Vec<u8>>) -> usize {
    let mut num_of_ones = 0;
    let mut num_of_twos = 0;

    for row in layer.iter() {
        for digit in row.iter() {
            match digit {
                1 => num_of_ones += 1,
                2 => num_of_twos += 1,
                _ => (),
            };
        }
    }
    num_of_ones * num_of_twos
}

fn draw_picture(layers: Vec<Vec<Vec<u8>>>) {
    for y in 0..Y_LEN {
        for x in 0..X_LEN {
            let mut layer_index = 0;
            while layers[layer_index][y][x] == 2 {
                layer_index += 1;
            }
            let pixel = layers[layer_index][y][x];
            print!("{}", if pixel == 0 {"█"} else {" "});
        }
        println!();
    }
}

fn main() {
    let digits = read_data();
    let (layers, index) = get_layers(digits);
    println!("{}", get_magic_number(layers[index].clone()));
    draw_picture(layers);
}