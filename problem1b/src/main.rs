use std::fs;
use std::io::{BufRead, BufReader};
// This code presents one solution to the 2015 advent of code problem from day 1: Santa and floor directions
// I am using this one to learn and practice Rust! 

fn main() {

    //We read in as text because copy and paste is broken for large input sometimes. 
    //Process: Read in string, parse to characters, iterate through to find when we enter the basement. 
    //Lets try an enum with the char iterable!

    let mut curr_floor: i32 = 0;

    let path = "./input.txt";
    let file = match fs::File::open(path) {
        Ok(file) => file,
        Err(_) => panic!("Unable to read title from {}", path),
    };
    let buffer = BufReader::new(file);   
    let directions = get_directions(buffer);

    for (i, c) in directions.chars().enumerate() {
        if c == '(' {
            curr_floor += 1;
        }
        if c == ')' {
            curr_floor -= 1;
        }
        if curr_floor < 0 {
            println!("{}",i+1);
            break;
        }
    }
}

fn get_directions<R>(mut rdr: R) -> String
    where R: BufRead,
    {
        let mut first_line = String::new();

        rdr.read_line(&mut first_line).expect("Unable to read line");

        return first_line
    }