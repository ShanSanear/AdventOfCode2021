use std::path::Path;
use crate::common::load_file;

mod day1;
mod common;
mod day2;


fn main() {
    let day1_base_out = day1::solve_day1_base();
    println!("Day 1 base out: {}", day1_base_out);
    let day1_add_out = day1::solve_day1_add();
    println!("Day 1 add out: {}", day1_add_out);
    let day2_base_out = day2::solve_day2_base();
    println!("Day 2 base out: {}", day2_base_out);
    let day2_add_out = day2::solve_day2_add(false);
    println!("Day 2 add out: {}", day2_add_out);
}
