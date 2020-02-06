use intcomputer::intcode;
use std::io;
use std::collections::{HashMap, HashSet};

const COMMAND: [char;8] = ['C','o','m','m','a','n','d','?'];

#[derive(Debug)]
pub enum Direction {
    North,
    South,
    West,
    East,
    Take,
    Drop,
    List,
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

pub struct Board {
    current_pos: Point,
    computer: intcode::Amplifier,
    current_item: String,
    items: HashSet<String>,
}

impl Board {
    pub fn new(computer: intcode::Amplifier) -> Board {
        let items = HashSet::new();
        let current_item = String::from("");
        Board { current_pos: Point{x: 0, y: 0}, computer: computer, current_item: current_item, items: items}
    }

    pub fn get_action(&self) -> Direction {
        let retval;
        loop {
            let mut input= String::new();
            io::stdin().read_line(&mut input)
                .expect("Failed to read line");

            if let Some(first_char) = input.chars().next() {
                retval = match first_char {
                    'u' => Direction::North,
                    'j' => Direction::South,
                    'h' => Direction::West,
                    'k' => Direction::East,
                    't' => Direction::Take,
                    'd' => Direction::Drop,
                    'l' => Direction::List,
                    _ => continue,
                };
                break;
            } else {
                continue
            };
        }
        retval
    }

    fn run_until_command(&mut self) -> (String, bool) {
        let mut instructions: String = String::new();
        let mut awaits_command = false;
        while let Some(c) = self.computer.run_program_until_output(false) {
            instructions.push(c as u8 as char);
            if instructions.ends_with("Command?") {
                awaits_command = true;
                break;
            }
        }
        (instructions, awaits_command)
    }

    pub fn start(&mut self) {
        let (instructions, awaits_command) = self.run_until_command();
        println!("{}", &instructions);
        if !awaits_command { std::process::exit(0) };
        self.parse_instructions(&instructions);
    }

    pub fn step(&mut self) {
        let dir = self.get_action();
        match dir {
            Direction::North => self.walk(dir),
            Direction::South => self.walk(dir),
            Direction::West => self.walk(dir),
            Direction::East => self.walk(dir),
            Direction::Drop => self.drop(),
            Direction::Take => self.take(),
            Direction::List => self.list(),
        }
    }

    fn walk(&mut self, dir: Direction) {
        match dir {
            Direction::North => self.computer.push_input_vec(vec!['n' as isize, 'o' as isize, 'r' as isize, 't' as isize, 'h' as isize, '\n' as isize]),
            Direction::South => self.computer.push_input_vec(vec!['s' as isize, 'o' as isize, 'u' as isize, 't' as isize, 'h' as isize, '\n' as isize]),
            Direction::West => self.computer.push_input_vec(vec!['w' as isize, 'e' as isize, 's' as isize, 't' as isize, '\n' as isize]),
            Direction::East => self.computer.push_input_vec(vec!['e' as isize, 'a' as isize, 's' as isize, 't' as isize, '\n' as isize]),
            _ => panic!("Wrong input for walk!"),
        }

        let (instructions, awaits_command) = self.run_until_command();
        println!("{}", &instructions);
        if !awaits_command { std::process::exit(0) };
        let walked = self.has_walked(&instructions);
        self.current_item = self.parse_instructions(&instructions);

        if walked {
            match dir {
                Direction::North => {self.current_pos.y -= 1},
                Direction::South => {self.current_pos.y += 1},
                Direction::West => {self.current_pos.x -= 1},
                Direction::East => {self.current_pos.x += 1},
                _ => panic!("Wrong input for walk!"),
            }
        }
    }

    fn has_walked(&mut self, instructions: &str) -> bool {
        true
    }


    fn list(&mut self) {
        self.computer.push_input_vec(to_vec(&format!("inv{}", "\n" )));
        let (instructions, awaits_command) = self.run_until_command();
        println!("{}", &instructions);
        if !awaits_command { std::process::exit(0) };

    }

    fn drop(&mut self) {
        println!("enter item to drop");
        let mut input= "".to_owned();
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");
        self.computer.push_input_vec(to_vec(&format!("drop {}{}", &input, "\n" )));
        let (instructions, awaits_command) = self.run_until_command();
        println!("{}", &instructions);
        if !awaits_command { std::process::exit(0) };

    }

    fn take(&mut self) {
        self.computer.push_input_vec(to_vec(&format!("take {}{}", &self.current_item, "\n" )));
        let (instructions, awaits_command) = self.run_until_command();
        println!("{}", &instructions);
        if !awaits_command { std::process::exit(0) };

    }

    fn parse_instructions(&mut self, instructions: &str) -> String {
        let mut lines = instructions.lines();
        let mut read_items = false;
        let mut current_item= String::from("");
        for line in lines {
            if line.contains("Items here:"){
                read_items = true;
                continue
            }
            if read_items {
                if line.contains("-") {
                    let item = line.trim_start_matches("-").trim().to_string();
                    current_item = item.clone();
                } else {
                    read_items = false
                }
            }
        }
        current_item
    }
}

fn parse_direction(direction: &str) -> Direction {
    match direction {
        "North" => Direction::North,
        "South" => Direction::South,
        "West" => Direction::West,
        "East" => Direction::East,
        _ => panic!("failed direction parse: {}", direction),
    }
}

fn did_not_hit_a_wall(instructions: &str) -> bool {
    true
}

//'t' => {
//computer.push_input_vec(vec!['t' as isize, 'a' as isize, 'k' as isize, 'e' as isize, ' ' as isize]);
//},
//'u' =>
//'j' =>
//'h' =>
//'k' =>
//'d' => {
//computer.push_input_vec(vec!['d' as isize, 'r' as isize, 'o' as isize, 'p' as isize, ' ' as isize]);
//},
//'l' => computer.push_input_vec(vec!['l' as isize, 'i' as isize, 's' as isize, 't' as isize, '\n' as isize, ]),
//_ => continue,

pub fn to_vec(s: &str) -> Vec<isize>{
    s.chars().map(|a| a as isize).collect()
}

pub fn ends_with_command(s: &str) -> bool {
    let len = COMMAND.len();
    let mut index = len - 1;

    for c in s.chars().rev() {
        if c != COMMAND[index] {
            return false;
        }
        if index == 0 {return true;}
        index -= 1;
    }
    false
}


#[cfg(test)]
mod tests {
    #[test]
    fn end_with_command() {
        assert!(String::from("Hello\
        Command?").ends_with("Command?"));
    }

    #[test]
    fn not_end_with_command() {
        assert!(!String::from("Command? ").ends_with("Command?"));
    }
}
