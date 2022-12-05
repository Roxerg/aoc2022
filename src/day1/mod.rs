use std::fs;


fn read_input() -> String {
    let contents = fs::read_to_string("day1/input.txt")
        .expect("Failed to read input");

    return contents;
}

fn read_2d_input() -> Vec<Vec<String>> {
    return read_input().split("\n\n").map(|x: &str| x.lines().map(|xx: &str| (*xx).to_string()).collect()).collect();
}

pub fn part1() {
    
    let inputs: Vec<Vec<String>> = read_2d_input();
    // let input1: Vec<String> = read_1d_input('\n');
    // let input_lines: Vec<Vec<&str>> =  raw_input.lines().map(|x: &str| x.split(',').collect()).collect();

    let mut sums: Vec<i32> = Vec::new();
    for input in &inputs {
        let sum: i32 = input.iter().fold(0, |sum, x| sum + x.parse::<i32>().unwrap());
        sums.push(sum);
    }

    let maxval: i32 = *sums.iter().max().unwrap();

    println!("{}", maxval);
}


pub fn part2() {
    
    let inputs: Vec<Vec<String>> = read_2d_input();

    let mut sums: Vec<i32> = Vec::new();
    for input in &inputs {
        let sum: i32 = input.iter().fold(0, |sum, x| sum + x.parse::<i32>().unwrap());
        sums.push(sum);
    }


    let mut res: i32 = 0;
    for _i in [1,2,3] {
        let maxval: i32 = *sums.iter().max().unwrap();
        sums.retain(|&x| x != maxval);
        res += maxval;

    }
    println!("{}", res);
}


