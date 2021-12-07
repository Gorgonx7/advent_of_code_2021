use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
fn main() {
    println!("Hello, world!");

    if let Ok(lines) = read_lines("./input.txt") {
        let mut last_number = 0;
        let mut last_last_number = 0;
        let mut current_increments = 0;
        let mut last_sum = 0;
        for line in lines {
            if let Ok(current) = line {
            let current_number = current.parse::<i32>().unwrap();
            
            if last_last_number == 0 {
                    last_last_number = current_number;
                    continue;
            }
            if last_number == 0 {
                last_number = current_number;
                continue;
            }
            if last_last_number + last_number + current_number > last_sum && last_sum != 0 {
                
                current_increments += 1;
            }
            last_sum = last_last_number + last_number + current_number;
            last_last_number = last_number;
            last_number = current_number;
            }
        }
        println!("{}", current_increments);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}