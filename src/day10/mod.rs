use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashSet;


use regex::Regex;

fn get_reader() -> BufReader<File> {
    let file = File::open("src/day9/input.txt").unwrap();
    let reader = BufReader::new(file);
    return reader;
}



pub fn part1() {
    return;
}
    

pub fn part2() {
    return;
}


