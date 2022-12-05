// use std::env;
use std::fs;


fn read_input() -> String {
    let contents = fs::read_to_string("day2/input.txt")
        .expect("Failed to read input");

    return contents;
}


fn read_2d_input() -> Vec<Vec<char>> {
    return read_input().lines().map(|x: &str| x.split(" ").map(|xx: &str| ((*xx).chars().collect::<Vec<char>>())[0]).collect()).collect();
}


fn count_score(to_beat: u32, selection: u32) -> u32 { 
    let mut score: u32 = 0;
    let mut to_beat_n = to_beat - 64;
    let mut selection_n =  selection - 64;
    score += selection_n;

    if to_beat_n == selection_n {
        return score + 3;
    }

    // if not equal and sum up to 4, flip the bitch (because rocks beat scissors)
    if to_beat_n + selection_n == 4 {
        let temp = to_beat_n;
        to_beat_n = selection_n;
        selection_n = temp;
    }
    
    if to_beat_n < selection_n {
        score += 6;
    } else if to_beat_n > selection_n {
        score += 0;
    }
    return score;
}

pub fn part1() {
    
    let inputs: Vec<Vec<char>> = read_2d_input();

    let mut total = 0;

    for input in &inputs {
        total += count_score(input[0] as u32, input[1] as u32);
    }


    println!("{}", total);
}


pub fn part2() {
    
    let inputs: Vec<Vec<char>> = read_2d_input();

    let mut total = 0;

    for input in &inputs {

        let mut selection: u32 = 0;

        // LOSE
        if input[1] == 'X' {
            if input[0] == 'A' {
                selection = 'C' as u32;
            } else {
                selection = input[0] as u32 - 1;
            }
        }

        // DRAW
        if input[1] == 'Y' {
            selection = input[0] as u32;
        }

        // WIN
        if input[1] == 'Z' {
            if input[0] == 'C' {
                selection = 'A' as u32;
            } else {
                selection = input[0] as u32 + 1;
            }
        }

        total += count_score(input[0] as u32, selection);
    }


    println!("{}", total);
}
