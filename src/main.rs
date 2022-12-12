#![feature(exclusive_range_pattern)]

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;

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

   println!("=== Day 5 ===");
   println!("Part 1: {:?}", day05::solve_part_one());
   println!("Part 2: {:?}", day05::solve_part_two());
   println!();

   println!("=== Day 6 ===");
   println!("Part 1: {:?}", day06::solve_part_one());
   println!("Part 2: {:?}", day06::solve_part_two());
   println!();

   println!("=== Day 7 ===");
   println!("Part 1: {:?}", day07::solve_part_one());
   println!("Part 2: {:?}", day07::solve_part_two());
   println!();

   println!("=== Day 8 ===");
   println!("Part 1: {:?}", day08::solve_part_one());
   println!("Part 2: {:?}", day08::solve_part_two());
   println!();

   println!("=== Day 9 ===");
   println!("Part 1: {:?}", day09::solve_part_one());
   println!("Part 2: {:?}", day09::solve_part_two());
   println!();

   println!("=== Day 10 ===");
   println!("Part 1: {}", day10::solve_part_one());
   println!("Part 2: <See CRT Output>\n\n{}", day10::solve_part_two());
   println!();
}
