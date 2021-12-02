use std::collections::HashMap;
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
    and save, using a dictionary, the values of forward, down and up.
    Next, we will do forward * (down - up) to get the correct result.
*/
fn main() {
    // Using a dictionary to update the values.
    let mut movements = HashMap::new();
    movements.insert("forward",0);
    movements.insert("down",0);
    movements.insert("up",0);

    // Read file
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let file = filename_to_string(&filename).unwrap();
    let wbyl = words_by_line(&file);

    // Process line by line.
    for words in wbyl {
        // We need to convert string to integer to make the sum.
        *movements.get_mut(words[0]).unwrap() += words[1].parse::<i32>().unwrap();
    }

    let result = movements["forward"] * (movements["down"] - movements["up"]);
    println!("{}", result);
}
