#[derive(Debug, Clone, Copy)]
struct Point {
   row: usize,
   col: usize,
}

#[derive(Debug)]
struct Tree {
   height: u32,
}

struct TreePatch {
   grid: Vec<Vec<Tree>>,
}

impl TreePatch {
   fn new(grid: Vec<Vec<Tree>>) -> Self {
      TreePatch { grid }
   }

   fn left_trees(&self, location: Point) -> Vec<&Tree> {
      self.grid[location.row]
         .iter()
         .take(location.col)
         .rev()
         .collect()
   }

   fn right_trees(&self, location: Point) -> Vec<&Tree> {
      self.grid[location.row]
         .iter()
         .skip(location.col + 1)
         .collect()
   }

   fn top_trees(&self, location: Point) -> Vec<&Tree> {
      self
         .grid
         .iter()
         .take(location.row)
         .map(|row| &row[location.col])
         .rev()
         .collect()
   }

   fn bottom_trees(&self, location: Point) -> Vec<&Tree> {
      self
         .grid
         .iter()
         .skip(location.row + 1)
         .map(|row| &row[location.col])
         .collect()
   }
}

fn parse_input() -> TreePatch {
   TreePatch::new(
      include_str!("../input/day08.txt")
         .lines()
         .map(|row| {
            row.chars()
               .map(|height| Tree {
                  height: height.to_digit(10).unwrap(),
               })
               .collect()
         })
         .collect(),
   )
}

fn is_visible(patch: &TreePatch, location: Point) -> bool {
   // All exterior trees are visible.
   if location.row == 0
      || location.row == patch.grid.len() - 1
      || location.col == 0
      || location.col == patch.grid[location.row].len() - 1
   {
      return true;
   }

   // Interior trees are visible if they are visible from any direction.
   let tree = &patch.grid[location.row][location.col];

   let is_visible_over =
      |trees: Vec<&Tree>| -> bool { trees.iter().all(|other| tree.height > other.height) };

   is_visible_over(patch.left_trees(location))
      || is_visible_over(patch.right_trees(location))
      || is_visible_over(patch.top_trees(location))
      || is_visible_over(patch.bottom_trees(location))
}

fn get_scenic_score(patch: &TreePatch, location: Point) -> usize {
   let tree = &patch.grid[location.row][location.col];

   let get_viewing_distance = |trees: Vec<&Tree>| {
      let mut viewing_distance = 0;

      for other in trees {
         viewing_distance += 1;
         if other.height >= tree.height {
            break;
         }
      }

      viewing_distance
   };

   get_viewing_distance(patch.left_trees(location))
      * get_viewing_distance(patch.right_trees(location))
      * get_viewing_distance(patch.top_trees(location))
      * get_viewing_distance(patch.bottom_trees(location))
}

pub fn solve_part_one() -> usize {
   let patch = parse_input();
   let mut visible_trees = 0;

   for row in 0..patch.grid.len() {
      for col in 0..patch.grid[row].len() {
         if is_visible(&patch, Point { row, col }) {
            visible_trees += 1;
         }
      }
   }

   visible_trees
}

pub fn solve_part_two() -> usize {
   let patch = parse_input();
   let mut highest_scenic_score = 0;

   for row in 0..patch.grid.len() {
      for col in 0..patch.grid[row].len() {
         let scenic_score = get_scenic_score(&patch, Point { row, col });
         if scenic_score > highest_scenic_score {
            highest_scenic_score = scenic_score;
         }
      }
   }

   highest_scenic_score
}
