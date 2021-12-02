use std::env;
use std::fs::File;
use std::io::{self, Read};

// Read all text into a file and save in a string.
fn filename_to_string(s: &str) -> io::Result<String> {
    let mut file = File::open(s)?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    Ok(s)
}

// Split a string between spaces
fn words_by_line<'a>(s: &'a str) -> Vec<Vec<&'a str>> {
    s.lines().map(|line| {
        line.split_whitespace().collect()
    }).collect()
}

/*
    The strategy will be to read the entire file, process it line by line, 
    and increase aim, h_position and depth acording to the values of forward, 
    down and up.
    Next, we will do h_position * depth to get the correct result.
*/
fn main() {
    // Read file from arg
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    
    let mut aim = 0;
    let mut h_position = 0;
    let mut depth = 0;

    let file = filename_to_string(&filename).unwrap();
    let wbyl = words_by_line(&file);

    // Process line by line
    for line in wbyl {
        let value = line[1].parse::<i32>().unwrap();

        if line[0] == "forward" {
            h_position += value;
            depth += value * aim;
        }
        else if line[0] == "up" {
            aim -= value; 
        }
        else {
            aim += value;
        }
    }

    let result = h_position * depth;
    println!("{}", result);
}
