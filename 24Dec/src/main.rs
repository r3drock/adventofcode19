use std::fs;
use std::collections::HashMap;

const X_LEN: usize = 5 + 2;
const Y_LEN: usize = 5 + 2;

fn read_tiles(path: &str) -> Vec<Vec<char>> {
    let data = fs::read_to_string(path)

        .expect("Something went wrong reading the file");

    let mut tiles: Vec<Vec<char>> = Vec::new();

    let mut y = 1;
    tiles.push(vec!['.', '.', '.', '.', '.', '.', '.']);
    for line in data.split('\n') {
        tiles.push(vec!['.']);
        for c in line.chars() {
            tiles[y].push(c);
        }
        tiles[y].push('.');
        y += 1;
    }
    tiles.push(vec!['.', '.', '.', '.', '.', '.', '.']);
    tiles
}

fn get_empty_tiles() -> Vec<Vec<char>> {
    let mut tiles = Vec::new();

    for y in 0..Y_LEN {
        tiles.push(vec!['.', '.', '.', '.', '.', '.', '.']);
    }

    tiles
}

fn print_tiles(tiles: &Vec<Vec<char>>) {
    for line in tiles {
        for c in line {
            print!("{}", c);
        }
        println!();
    }
    println!();
}

fn hash(tiles: &Vec<Vec<char>>) -> u32 {
    let mut hash = 0;
    let mut bit = 1;

    for y in 1..(Y_LEN-1) {
        for x in 1..(X_LEN-1) {
            if tiles[y][x] == '#' {
                hash |= bit;
            }
            bit <<= 1;
        }
    }

    hash
}

fn iterate1_minute(tiles: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut result = get_empty_tiles();

    for y in 1..(Y_LEN-1) {
        for x in 1..(X_LEN-1) {
            let mut num_adjacent_tiles = 0;
            num_adjacent_tiles += if tiles[y - 1][x] == '#' { 1 } else { 0 };
            num_adjacent_tiles += if tiles[y + 1][x] == '#' { 1 } else { 0 };
            num_adjacent_tiles += if tiles[y][x - 1] == '#' { 1 } else { 0 };
            num_adjacent_tiles += if tiles[y][x + 1] == '#' { 1 } else { 0 };
            if tiles[y][x] == '#' {
                if num_adjacent_tiles == 1 {
                    result[y][x] = '#';
                }
            } else if tiles[y][x] == '.'{
                if num_adjacent_tiles == 1 || num_adjacent_tiles == 2 {
                    result[y][x] = '#';
                }
            }
        }
    }
    result
}

fn part1() {
    let mut hashes: HashMap<u32, ()> = HashMap::new();

    let mut tiles = read_tiles("input");

    loop {
        let hash_val = hash(&tiles);
        if hashes.contains_key(&hash_val) {
            println!("{}", hash_val);
            break;
        } else {
            hashes.insert(hash_val, ());
            tiles = iterate1_minute(tiles);
        }
    }
}

fn main() {
    part1();
}
