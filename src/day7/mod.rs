use std::fs::File;
use std::hash::Hash;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashMap;
use std::ops::Deref;
use std::str::FromStr;


use regex::Regex;

fn get_reader() -> BufReader<File> {
    let file = File::open("src/day7/input.txt").unwrap();
    let reader = BufReader::new(file);
    return reader;
}


fn do_the_thang() -> HashMap<String, u32> {

    let mut reader: BufReader<File> = get_reader();
    let mut fs: HashMap<String, u32> = HashMap::new();

    let mut curr_line: String = String::new();
    let mut curr_path: Vec<String> = vec!();

    curr_line.clear();
    let mut size = reader.read_line(&mut curr_line).expect("yeet");
    
    while size > 0 {
        let curr_line_cp = curr_line.clone();
        let cmd: Vec<&str> = curr_line_cp.split(' ').map(|x| x.trim()).collect();

        if cmd[1].eq("cd") {
            curr_line.clear();
            size = reader.read_line(&mut curr_line).expect("yeet");
            if cmd[2].eq("..") {
                curr_path.pop();
                continue;
            }
            curr_path.push(cmd[2].to_string());
            if !fs.contains_key(&curr_path.join("/")) {
                fs.insert(
                    curr_path.join("/"),
                    0
                );
            }
        } else if cmd[1].eq("ls") {
            curr_line.clear();
            size = reader.read_line(&mut curr_line).expect("yeet");
            while curr_line.len() > 1 && curr_line.chars().nth(0).unwrap() != '$' {
                let content: Vec<&str> = curr_line.split(' ').collect();
                
                if !content[0].eq("dir") {
                    let n: u32 = FromStr::from_str(content[0]).unwrap();
                    *fs.entry(curr_path.join("/")).or_insert(0) += n;
                }

                
                curr_line.clear();
                size = reader.read_line(&mut curr_line).expect("yeet");
            }
        } else {
            size = reader.read_line(&mut curr_line).expect("yeet");
        }
    }


    return fs;

}


pub fn part1() {
    let fs = do_the_thang(); 

    let mut total = 0;
    for (key, init_val) in &fs {
        let re = Regex::new(&format!("{}.+", regex::escape(key))).unwrap();
        let mut dir_sum = 0 + init_val;
        for inner_key in fs.keys().filter(|x| re.is_match(x)) {
            dir_sum += fs[inner_key];
        }
        if dir_sum <= 100000 {
            total += dir_sum;
        }
    }
    println!("total = {}", total);
}
    

pub fn part2() {
    let fs = do_the_thang();

    let re = Regex::new(&format!("{}.+", regex::escape("/"))).unwrap();
    let mut fs_size = 0 + fs["/"];
    for inner_key in fs.keys().filter(|x| re.is_match(x)) {
        fs_size += fs[inner_key];
    }

    let thresh = 30000000 - (70000000 - fs_size);
    let mut min = thresh*2;

    let mut total = 0;
    for (key, init_val) in &fs {
        let re = Regex::new(&format!("{}.+", regex::escape(key))).unwrap();
        let mut dir_sum = 0 + init_val;
        for inner_key in fs.keys().filter(|x| re.is_match(x)) {
            dir_sum += fs[inner_key];
        }
        if dir_sum > thresh && dir_sum < min {
            min = dir_sum;
        }
    }
    println!("smallest needed = {}", min);


    return;
}
