// use std::env;
use std::fs;
use std::collections::HashSet;

fn read_input() -> String {
    return fs::read_to_string("day3/input.txt").expect("Failed to read input");
}

fn read_1d_input(delim: char) -> Vec<String> {
    return read_input().split(|a: char| a.eq(&delim)).map(|x: &str| (*x).to_string()).collect();
}


fn score_calc(n: u32) -> u32 {
    if n > 95 { n - 96} else { n - 65 + 27 }
}


pub fn part1() {
    
    let inputs: Vec<String> = read_1d_input('\n');

    let mut total = 0;

    for input in &inputs {
       let half_idx = input.chars().count() / 2;
       let first = &input[..half_idx];
       let second = &input[half_idx..];
       let mut done = false;
       for char1 in first.chars() {
           for char2 in second.chars() {
               if char1 == char2 {
                  let score = score_calc(char1 as u32);
                  done = true;
                  total += score;
                  break;
               }
           }
           if done {
            break;
           }
       }
    }

    println!("{}", total);   
}


pub fn part2() {
    
    let inputs: Vec<String> = read_1d_input('\n');
    let mut total = 0;

    for input_idx in (0..inputs.len()-1).step_by(3) {

        let input = &inputs[input_idx..input_idx+3];

        let mut match_set: HashSet<char> = input[0].chars().collect();
        match_set = input[1].chars().filter(|c| match_set.contains(&c)).collect();
        let common_char: char = input[2].chars().find(|c| match_set.contains(&c)).unwrap();

        total += score_calc(common_char as u32);
    
    }

    println!("{}", total);
}

