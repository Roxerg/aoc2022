// use std::env;
use std::fs;


fn read_input() -> String {
    let contents = fs::read_to_string("day6/input.txt")
        .expect("Failed to read input");
    return contents;
}


fn find_idx<const BUFFER_SIZE: usize>() {
    let mut buffer: [char; BUFFER_SIZE] = ['0'; BUFFER_SIZE];
    let mut max_i = 0;
    for (i, c) in read_input().chars().enumerate() {
        buffer[i % BUFFER_SIZE] = c;
        max_i = i+1;
        if i > 2 {
            let has_dups = (0..buffer.len()).any(|i| buffer[i+1..].contains(&buffer[i]));
            if !has_dups {
                break;
            }
        }
    }
    println!("{}", max_i);
}


pub fn part1() {
    find_idx::<4>();
}
    

pub fn part2() {
    find_idx::<14>();
}
