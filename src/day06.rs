use std::collections::HashSet;

fn parse_input() -> Vec<char> {
   include_str!("../input/day06.txt").chars().collect()
}

fn find_marker_position(buffer: &Vec<char>, window_size: usize) -> Option<usize> {
   let sequences = buffer.windows(window_size);

   for (marker, sequence) in sequences.enumerate() {
      if sequence.iter().collect::<HashSet<_>>().len() == window_size {
         return Some(marker + window_size);
      }
   }

   None
}

pub fn solve_part_one() -> usize {
   let buffer = parse_input();
   find_marker_position(&buffer, 4).unwrap()
}

pub fn solve_part_two() -> usize {
   let buffer = parse_input();
   find_marker_position(&buffer, 14).unwrap()
}
