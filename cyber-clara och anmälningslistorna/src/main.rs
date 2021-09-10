use std::io;
use std::io::Read;
use std::io::prelude::*;
use std::collections::HashSet;

fn main() {
    // We get the input
    let input = io::stdin();
    let mut _input = input.lock();
    let mut s: String = String::new();
    let _ = _input.read_to_string(&mut s);
    let mut lines = s.lines();

    // Create both a variable for the number of lines Kattis tells us the file includes
    // as well as the container for our names.
    let x: usize = lines.next().unwrap().parse().unwrap();
    let mut names = Vec::with_capacity(x);
    
    // Take all the first names, and add them to the vector
    for number in 1..=x {
        names.push(lines.next().unwrap());
    }

    // Zip up the first names with last names into a hashset, getting only uniques, then print length.
    let mut h = HashSet::with_capacity(x);
    h.extend(names.into_iter().zip(lines));
    println!("{}", h.len());
}