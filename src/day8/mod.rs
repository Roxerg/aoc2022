use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashSet;


use regex::Regex;

fn get_reader() -> BufReader<File> {
    let file = File::open("src/day8/input.txt").unwrap();
    let reader = BufReader::new(file);
    return reader;
}



pub fn part1() {

    let mut reader = get_reader();
    let mut curr_line: String = String::new();

    let mut seen = HashSet::<u32>::new();
    let mut total_idx = 0;

    curr_line.clear();
    let mut size = reader.read_line(&mut curr_line).expect("yeet");
    let mut max_vert:Vec<char> = vec!['+'; size-1];

    let mut columns: Vec<Vec<char>> = vec![vec!(); size-1];

    let mut row_idx = 0;
    while size > 0 {
        let plain_line =  curr_line.strip_suffix("\n").unwrap_or(curr_line.as_str());
        let mut line_idx: u32 = total_idx;
        let mut max_line = '+';
        for (col, tree) in plain_line.chars().enumerate() {

            println!("idx: {} val: {}", total_idx, tree);

            columns[col as usize].push(tree);

            if tree > max_line {
                max_line = tree;
                if !seen.contains(&line_idx) {
                    seen.insert(line_idx);
                    // println!("added {} nth going left", line_idx);
                }
            }
            if tree > max_vert[col as usize] {
                // println!("{} > {}", tree, max_vert[col as usize]);
                max_vert[col as usize] = tree;
                if !seen.contains(&line_idx) {
                    seen.insert(line_idx);
                    // println!("added {} nth from vert going down", line_idx);
                }
            }
            total_idx += 1;
            line_idx += 1;
        }              


        max_line = '+';
        line_idx -= 1;
        //total_idx -= 1;

        for tree in plain_line.chars().rev() {
            if tree > max_line {
                max_line = tree;
                if !seen.contains(&line_idx) {
                    seen.insert(line_idx);
                    // println!("added {} nth going right", line_idx);
                }
            }
            if line_idx > 0 {
                line_idx -= 1;
            }   
        }

        curr_line.clear();
        size = reader.read_line(&mut curr_line).expect("yeet");   
        row_idx += 1;
    }

    println!("zzzzzzzzz");

    let idx_offset = columns[0].len() as u32;

    // total_idx -= 1;
    // println!("offset {}", idx_offset);

    let mut total_idx_cp = total_idx - 1;
    for col in columns.iter().rev() {
        let mut max_col = '+';
        let mut col_elem_idx = total_idx_cp;
        for tree in col.iter().rev() {

            println!("idx: {} val: {}", col_elem_idx, tree);

            if *tree >= max_col {
                max_col = *tree;
                if !seen.contains(&col_elem_idx) {
                    seen.insert(col_elem_idx);
                    // println!("added {} nth from vert going up", col_elem_idx);
                }
            }
            if (col_elem_idx >= idx_offset) {
                col_elem_idx -= idx_offset;
            }
        }
        total_idx_cp -= 1;
    }
   
    println!("{}", seen.len());
    
    return;
}
    

pub fn part2() {
    return;
}



/*  
A B C D
E F G H
I J K L
M N O P

 0  1  2  3  4
 5  6  7  8  9
10 11 12 13 14
15 16 17 18 19
20 21 22 23 24




3 0 3 7 3
2 5 5 1 2
6 5 3 3 2
3 3 5 4 9
3 5 3 9 0
*/
