type Calories = u32;

fn parse_input() -> Vec<Vec<Calories>> {
   include_str!("../input/day01.txt")
      .split("\n\n")
      .map(|food_bags| {
         food_bags
            .lines()
            .map(|calories| calories.parse().expect("invalid calorie value"))
            .collect()
      })
      .collect()
}

pub fn solve_part1() -> Calories {
   parse_input()
      .iter()
      .map(|food_bag| food_bag.iter().sum())
      .max()
      .expect("no food bags")
}

pub fn solve_part2() -> Calories {
   let mut food_bags: Vec<Calories> = parse_input()
      .iter()
      .map(|food_bag| food_bag.iter().sum())
      .collect();

   food_bags.sort_by(|a, b| b.cmp(a));
   food_bags.iter().take(3).sum()
}
