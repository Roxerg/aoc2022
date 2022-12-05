use std::env;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

fn main() {

    let args: Vec<String> = env::args().collect();
    let day: u32 = args[1].parse().unwrap();

    match day {
        1 => {
            day1::part1();
            day1::part2();
        }, 
        2 => {
            day2::part1();
            day2::part2();
        },
        3 => {
            day3::part1();
            day3::part2();
        },
        4 => {
            day4::part1();
            day4::part2();
        },
        5 => {
            day5::part1();
            day5::part2();
        },
        0_u32 | 6_u32..=u32::MAX => todo!()
    }

    
}

