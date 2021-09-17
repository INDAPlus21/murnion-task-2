use std::io;
use std::io::prelude::*;
use std::cmp;

/// Takes an input file with two numbers R and C, in a format of a single line "r c"
/// Then converts it into an output string displaying a rectangle R wide and C long
fn main() {
    // Get the input
    let input = io::stdin(); 
    let mut s = &input.lock().lines().map(|_line| _line.ok().unwrap()).collect::<Vec<String>>()[0];
    let mut values = s.split(" ");

    // Break down the input into the actual values we want
    let rows: usize = values.next().unwrap().parse().unwrap();
    let columns: usize = values.next().unwrap().parse().unwrap();
    
    // The rows part just prints it and initializes the string.
    for x in 1..=rows {
        let mut name: String = String::with_capacity(columns);
        // The columns part constructs the string using the dist_map function.
        for y in 1..=columns {
            name.push(dist_map(cmp::min(cmp::min(x, rows - x + 1), cmp::min(y, columns - y + 1))));
        }
        println!("{}", name);
    }
}

/// Map numbers to chars, where any number above 9 is a dot.
fn dist_map(x: usize) -> char {
    match x {
        1 => '1',
        2 => '2',
        3 => '3',
        4 => '4',
        5 => '5',
        6 => '6',
        7 => '7',
        8 => '8',
        9 => '9',
        _ => '.',
    }
}