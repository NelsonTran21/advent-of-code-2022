mod day01;
mod day02;

fn main() {
   println!("=== Day 1 ===");
   println!("Part 1: {:?}", day01::solve_part_one());
   println!("Part 2: {:?}", day01::solve_part_two());
   println!();

   println!("=== Day 2 ===");
   println!("Part 1: {:?}", day02::solve_part_one());
   println!("Part 2: {:?}", day02::solve_part_two());
   println!();
}
