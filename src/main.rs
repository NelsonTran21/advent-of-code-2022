#![feature(exclusive_range_pattern)]

mod day01;
mod day02;
mod day03;
mod day04;

fn main() {
   println!("=== Day 1 ===");
   println!("Part 1: {:?}", day01::solve_part_one());
   println!("Part 2: {:?}", day01::solve_part_two());
   println!();

   println!("=== Day 2 ===");
   println!("Part 1: {:?}", day02::solve_part_one());
   println!("Part 2: {:?}", day02::solve_part_two());
   println!();

   println!("=== Day 3 ===");
   println!("Part 1: {:?}", day03::solve_part_one());
   println!("Part 2: {:?}", day03::solve_part_two());
   println!();

   println!("=== Day 4 ===");
   println!("Part 1: {:?}", day04::solve_part_one());
   println!("Part 2: {:?}", day04::solve_part_two());
   println!();
}
