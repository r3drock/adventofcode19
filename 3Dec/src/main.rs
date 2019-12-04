use std::fs;

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

fn calculate_dimensions (cable: Vec<(&str, &str)>) {
    
}

fn main() {
    let mut cable1: Vec<(&str, &str)> = Vec::new();
    let mut cable2: Vec<(&str, &str)> = Vec::new();
    let data = fs::read_to_string("data")
        .expect("Something went wrong reading the file");
    let mut lines = data.split('\n');
    let cable1_directions:String = lines.next()
        .expect("Line 1 is missing.").to_string();
    let cable2_directions:String = lines.next()
        .expect("Line 2 is missing.").to_string();
    for direction in cable1_directions.split(',') {
        cable1.push((&direction[..1], &direction[1..]));
    }
    for direction in cable2_directions.split(',') {
        cable2.push((&direction[..1], &direction[1..]));
    }





    for (direction, length) in cable1 {
        println!("({}, {})", direction, length);
    }
        println!("--------------------");
    for (direction, length) in cable2 {
        println!("({}, {})", direction, length);
    }
}
