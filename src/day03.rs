use std::collections::HashSet;

type Item = char;

type Priority = u32;

#[derive(Debug)]
struct Rucksack(HashSet<Item>, HashSet<Item>);

fn parse_input() -> Vec<Rucksack> {
   include_str!("../input/day03.txt")
      .lines()
      .map(|line| {
         let length = line.chars().count();
         let (first, last) = line.split_at(length / 2);
         Rucksack(first.chars().collect(), last.chars().collect())
      })
      .collect()
}

fn get_priority(item: Item) -> Priority {
   match item {
      'a'..='z' => u32::from(item) - 97 + 1,
      'A'..='Z' => u32::from(item) - 65 + 27,
      _ => {
         println!("{:?}", item);
         unreachable!()
      }
   }
}

fn find_matching_items_in_rucksack(rucksack: Rucksack) -> HashSet<Item> {
   rucksack
      .0
      .intersection(&rucksack.1)
      .map(|item| item.to_owned())
      .collect()
}

fn find_matching_items_in_rucksacks(
   one: &Rucksack,
   two: &Rucksack,
   three: &Rucksack,
) -> HashSet<Item> {
   let all_one: HashSet<Item> = one.0.union(&one.1).into_iter().copied().collect();
   let all_two: HashSet<Item> = two.0.union(&two.1).into_iter().copied().collect();
   let all_three: HashSet<Item> = three.0.union(&three.1).into_iter().copied().collect();

   all_one
      .intersection(&all_two)
      .into_iter()
      .copied()
      .collect::<HashSet<_>>()
      .intersection(&all_three)
      .into_iter()
      .copied()
      .collect()
}

pub fn solve_part_one() -> Priority {
   let rucksacks = parse_input();

   rucksacks
      .into_iter()
      .map(|rucksack| -> Priority {
         find_matching_items_in_rucksack(rucksack)
            .into_iter()
            .map(get_priority)
            .sum()
      })
      .sum()
}

pub fn solve_part_two() -> Priority {
   let rucksacks = parse_input();
   let rucksack_chunks: Vec<&[Rucksack]> = rucksacks.chunks(3).collect();
   rucksack_chunks
      .into_iter()
      .map(|rucksacks| -> Priority {
         find_matching_items_in_rucksacks(&rucksacks[0], &rucksacks[1], &rucksacks[2])
            .into_iter()
            .map(get_priority)
            .sum()
      })
      .sum()
}
