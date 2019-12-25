use std::fs;
use std::fmt;
use std::fmt::Debug;

#[derive(Debug)]
struct Vector {
    x: i64,
    y: i64,
    z: i64,
}

struct Moon {
    pos: Vector,
    vel: Vector,
}

impl fmt::Display for Moon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "pos=<{:3}, {:3}, {:3}>, vel=<{:3}, {:3}, {:3}>",
            self.pos.x, self.pos.y, self.pos.z,
            self.vel.x, self.vel.y, self.vel.z)
    }
}

fn read_data(file_name: &str) -> Vec<Moon> {
    let data: String =
        fs::read_to_string(file_name).expect("Something went wrong reading the file");
    let mut moons: Vec<Moon> = Vec::new();

    for line in data.split('\n') {
        let mut value_iter = line.split('>').next().unwrap().split(',');
        let moon = Moon {
            pos: Vector {
                x: value_iter.next().unwrap()[3..].parse::<i64>().unwrap(),
                y: value_iter.next().unwrap()[3..].parse::<i64>().unwrap(),
                z: value_iter.next().unwrap()[3..].parse::<i64>().unwrap(),
            },
            vel: Vector { x: 0, y: 0, z: 0 },
        };
        moons.push(moon);
    }
    moons
}

#[allow(dead_code)]
fn print_moons(moons: &Vec<Moon>, iteration_count: u64) {
    println!("Afer {} steps:", iteration_count);
    for moon in moons {
        println!("{}", moon);
    }
}

fn iteration(moons: &mut Vec<Moon>) {
    let size = moons.len();

    for i in 0..size {
        for j in 0..size {
            if i == j {continue}
            moons[i].vel.x += if moons[i].pos.x < moons[j].pos.x {1} else if moons[i].pos.x == moons[j].pos.x {0} else {-1};
            moons[i].vel.y += if moons[i].pos.y < moons[j].pos.y {1} else if moons[i].pos.y == moons[j].pos.y {0} else {-1};
            moons[i].vel.z += if moons[i].pos.z < moons[j].pos.z {1} else if moons[i].pos.z == moons[j].pos.z {0} else {-1};
        }
    }
    for i in 0..size {
        moons[i].pos.x += moons[i].vel.x;
        moons[i].pos.y += moons[i].vel.y;
        moons[i].pos.z += moons[i].vel.z;
    }
}


fn potential_energy(moon: &Moon) -> u64 {
    (moon.pos.x.abs() + moon.pos.y.abs() + moon.pos.z.abs()) as u64
}

fn kinetic_energy(moon: &Moon) -> u64 {
    (moon.vel.x.abs() + moon.vel.y.abs() + moon.vel.z.abs()) as u64
}

fn total_energy(moons: &Vec<Moon>) -> u64 {
    let mut energy = 0;
    for moon in moons {
        energy += kinetic_energy(moon) * potential_energy(moon);
    }
    energy
}

fn main() {
    let mut moons = read_data("data");
    const ITERATIONS: u64 = 1000;
    for _ in 0..ITERATIONS {
        iteration(&mut moons);
    }
    println!("{}", total_energy(&moons));
}
