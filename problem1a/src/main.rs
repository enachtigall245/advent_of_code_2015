use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader};
// This code presents one solution to the 2015 advent of code problem from day 1: Santa and floor directions
// I am using this one to learn and practice Rust! 

fn main() {

    //We are just going to copy and paste in the directions because I am lazy (I could make it read in from text but thats a pain)
    //Process: Read in string, parse to characters, make a hashmap of characters
    //Floor increments are then just the character counts in the hashmap. Add and subtract as needed + print

    let mut curr_floor: i32 = 0;
    let mut letter_counts: HashMap<char,i32> = HashMap::new();

    let path = "./input.txt";
    let file = match fs::File::open(path) {
        Ok(file) => file,
        Err(_) => panic!("Unable to read title from {}", path),
    };
    let buffer = BufReader::new(file);   
    let directions = get_directions(buffer);

    let char_vec: Vec<char> = directions.to_lowercase().chars().collect();

    for c in char_vec {
        *letter_counts.entry(c).or_insert(0) += 1;
    }

    let open_exists = letter_counts.contains_key(&'(');
    let close_exists = letter_counts.contains_key(&')');

    if open_exists {
        curr_floor += letter_counts[&'('];
    }
    if close_exists {
        curr_floor -= letter_counts[&')'];
    }

    println!("floor {}",curr_floor);
}

fn get_directions<R>(mut rdr: R) -> String
    where R: BufRead,
    {
        let mut first_line = String::new();

        rdr.read_line(&mut first_line).expect("Unable to read line");

        return first_line
    }