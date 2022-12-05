// use std::env;
use std::fs;
use regex::Regex;


fn read_input() -> String {
    let contents = fs::read_to_string("day5/input.txt")
        .expect("Failed to read input");
    return contents;
}

fn just_do_the_whole_thing(reversy: bool) {
    let get_boxes: Regex = Regex::new(r"(\[(.*?)\]| {4})").unwrap();
    let get_stack_nos: Regex = Regex::new(r"[0-9]*").unwrap();
    let get_cmd_nos = Regex::new(r"[0-9]*").unwrap();

    let input = read_input().split("\n\n").map(|s| s.to_string()).collect::<Vec<String>>();
    
    let boxes_str: &str = input[0].as_str();
    let cmds_str: &str = input[1].as_str();


    let max: usize = get_stack_nos.find_iter(boxes_str).filter_map(|digits| digits.as_str().parse().ok()).max().unwrap();
    let mut stacks:Vec<Vec<String>> = vec![vec!(); max as usize];

    let boxes = get_boxes
        .find_iter(boxes_str)
        .filter_map(|b| b.as_str().parse().ok()).collect::<Vec<String>>();
        
    for (i, b) in boxes.iter().enumerate() {

        if b.chars().next().unwrap() == '[' {
            stacks[i % max].push(b.chars().skip(1).take(1).collect());
        }
    }
    
    {
        for s in &mut stacks {
            s.reverse();
        }
    }

    for cmd_str in cmds_str.split('\n') {
        let cmd: Vec<usize> = get_cmd_nos.find_iter(cmd_str).filter_map(|digits| digits.as_str().parse().ok()).collect();
        
        let box_no = cmd[0];
        let from_stack = cmd[1]-1;
        let to_stack = cmd[2]-1;

        let remaining = stacks[from_stack].len()-box_no;
        let mut chunk = stacks[from_stack].split_off(remaining);
        if reversy {
            chunk.reverse();
        }
        stacks[to_stack].append(&mut chunk);
    }


    for bb in &stacks {
        print!("{}", bb.last().unwrap());
    }
    
    println!();

}



pub fn part1() {
    just_do_the_whole_thing(true);
}

pub fn part2() {
    just_do_the_whole_thing(false);
}
