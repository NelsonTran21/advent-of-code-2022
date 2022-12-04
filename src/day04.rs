#[derive(Debug)]
struct Assignment {
   start: u8,
   end: u8,
}

impl Assignment {
   fn fully_contains(self: &Self, other: &Assignment) -> bool {
      self.start <= other.start && self.end >= other.end
   }

   fn overlaps_with(self: &Self, other: &Assignment) -> bool {
      (self.start >= other.start && self.start <= other.end)
         || (self.end >= other.start && self.end <= other.end)
         || (other.start >= self.start && other.start <= self.end)
         || (other.end >= self.start && other.end <= self.end)
   }
}

#[derive(Debug)]
struct AssignmentPair {
   first: Assignment,
   second: Assignment,
}

fn parse_assignment(assignment: &'static str) -> Assignment {
   assignment
      .split_once("-")
      .and_then(|(start, end)| {
         Some(Assignment {
            start: start.parse().unwrap(),
            end: end.parse().unwrap(),
         })
      })
      .unwrap()
}

fn parse_input() -> Vec<AssignmentPair> {
   include_str!("../input/day04.txt")
      .lines()
      .map(|line| {
         let (first_elf, second_elf) = line.split_once(",").unwrap();
         AssignmentPair {
            first: parse_assignment(first_elf),
            second: parse_assignment(second_elf),
         }
      })
      .collect()
}

pub fn solve_part_one() -> usize {
   parse_input()
      .iter()
      .filter(|p| p.first.fully_contains(&p.second) || p.second.fully_contains(&p.first))
      .count()
}

pub fn solve_part_two() -> usize {
   parse_input()
      .iter()
      .filter(|p| p.first.overlaps_with(&p.second))
      .count()
}
