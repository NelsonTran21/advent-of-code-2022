type Calories = u32;

fn parse_input() -> Vec<Calories> {
   include_str!("../input/day01.txt")
      .split("\n\n")
      .map(|food_bags| {
         food_bags
            .lines()
            .map(|food| food.parse::<Calories>().expect("bad value"))
            .sum()
      })
      .collect()
}

pub fn solve_part1() -> Calories {
   let food_bags = parse_input();
   food_bags.into_iter().max().expect("no food bags")
}

pub fn solve_part2() -> Calories {
   let mut food_bags = parse_input();
   food_bags.sort_by(|a, b| b.cmp(a));
   food_bags.into_iter().take(3).sum()
}
