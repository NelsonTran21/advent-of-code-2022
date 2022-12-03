use std::collections::HashSet;

type Item = char;

type Priority = u32;

struct Rucksack(HashSet<Item>, HashSet<Item>);

impl Rucksack {
   fn all_items(&self) -> HashSet<Item> {
      self.0.union(&self.1).copied().collect()
   }
}

fn parse_input() -> Vec<Rucksack> {
   include_str!("../input/day03.txt")
      .lines()
      .map(|line| {
         let rucksack_size = line.chars().count();
         let (first, second) = line.split_at(rucksack_size / 2);
         Rucksack(first.chars().collect(), second.chars().collect())
      })
      .collect()
}

fn get_priority(item: Item) -> Priority {
   match item {
      'a'..='z' => u32::from(item) - 97 + 1,
      'A'..='Z' => u32::from(item) - 65 + 27,
      _ => unreachable!(),
   }
}

fn find_matching_item(rucksack: Rucksack) -> HashSet<Item> {
   rucksack.0.intersection(&rucksack.1).copied().collect()
}

fn find_group_badge(rucksacks: &[Rucksack]) -> HashSet<Item> {
   rucksacks
      .iter()
      .skip(1)
      .fold(rucksacks[0].all_items(), |acc, rucksack| {
         acc.intersection(&rucksack.all_items()).copied().collect()
      })
}

pub fn solve_part_one() -> Priority {
   parse_input()
      .into_iter()
      .map(|rucksack| -> Priority {
         find_matching_item(rucksack)
            .into_iter()
            .map(get_priority)
            .sum()
      })
      .sum()
}

pub fn solve_part_two() -> Priority {
   parse_input()
      .chunks(3)
      .into_iter()
      .map(|rucksacks| -> Priority {
         find_group_badge(rucksacks)
            .into_iter()
            .map(get_priority)
            .sum()
      })
      .sum()
}
