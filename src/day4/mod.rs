use std::fs;

fn read_input() -> String {
    return fs::read_to_string("day4/input.txt").expect("Failed to read input");
}

fn read_2d_input() -> Vec<Vec<String>> {
    return read_input().split("\n").map(|x: &str| x.split(',').map(|xx: &str| (*xx).to_string()).collect()).collect();
}

fn get_range_vec(val: &String) -> Vec<u32> {
    return val.split('-').map(|xx: &str| (*xx).parse::<u32>().unwrap()).collect();
}

fn x_overlaps_y(x: &Vec<u32>, y: &Vec<u32>) -> bool {
    return ((x[0] <= y[0]) && (y[1] <= x[1])) 
        || (x[0] <= y[0]) && (x[1] <= y[1]) && (y[0] <= x[1])
}

pub fn part1() {
    
    let mut total = 0;
    let inputs: Vec<Vec<String>> = read_2d_input();

    for input in inputs {
        let range1: Vec<u32> = input[0].split('-').map(|xx: &str| (*xx).parse::<u32>().unwrap()).collect();
        let range2: Vec<u32> = input[1].split('-').map(|xx: &str| (*xx).parse::<u32>().unwrap()).collect();

        if (range1[0] <= range2[0]) && (range2[1] <= range1[1]) {
            // println!("{} {}", input[0], input[1]);
            total += 1;
        } else if  (range2[0] <= range1[0]) && (range1[1] <= range2[1]) {
            // println!("{} {}", input[0], input[1]);
            total += 1;
        }

    }

    println!("{}", total);

   
}

pub fn part2() {
    
    let mut total = 0;
    let inputs: Vec<Vec<String>> = read_2d_input();

    for input in inputs {
        let range1: Vec<u32> = get_range_vec(&input[0]);
        let range2: Vec<u32> = get_range_vec(&input[1]);

        if x_overlaps_y(&range1, &range2) || x_overlaps_y(&range2, &range1) {
            total += 1;
        }
    }

    println!("{}", total);
}

