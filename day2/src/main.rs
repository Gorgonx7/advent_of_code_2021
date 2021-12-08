use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
pub enum StringEnum {
    Forward,
    Up,
    Down,
}
fn main() {
    let mut current_depth = 0;
    let mut current_distance = 0;
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(current) = line {
                let mut iter = current.split_whitespace();
                let direction = iter.next().unwrap();
                let space = iter.next().unwrap();
                println!("{}", direction);

                match direction {
                    "up" => current_depth += space.parse::<i32>().unwrap(),
                    "forward" => current_distance += space.parse::<i32>().unwrap(),
                    "down" => current_depth -= space.parse::<i32>().unwrap(),
                    _ => println!("blahh blahhh"),
                }
            }
             println!("{}", current_depth);
             println!("{}", current_distance);
        }
        println!("{}", current_depth);
        println!("{}", current_distance);
        println!("{}", current_depth * current_distance);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}