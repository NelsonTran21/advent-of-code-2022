use std::{collections::HashSet, fmt::Display, thread::sleep, time::Duration};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
   x: i32,
   y: i32,
}

#[derive(Debug)]
struct SimulationState {
   visited: HashSet<Point>,
   head: Point,
   tail: Vec<Point>,
}

impl SimulationState {
   fn new(rope_size: usize) -> Self {
      if rope_size < 2 {
         panic!("Rope size must be at least 2");
      }

      SimulationState {
         visited: HashSet::new(),
         head: Point { x: 0, y: 0 },
         tail: vec![Point { x: 0, y: 0 }; rope_size - 1],
      }
   }
}

impl Display for SimulationState {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      let all_points = self
         .tail
         .iter()
         .chain(std::iter::once(&self.head))
         .chain(self.visited.iter())
         .cloned()
         .collect::<HashSet<_>>();

      let min_x = all_points.iter().map(|p| p.x).min().unwrap();
      let max_x = all_points.iter().map(|p| p.x).max().unwrap();
      let min_y = all_points.iter().map(|p| p.y).min().unwrap();
      let max_y = all_points.iter().map(|p| p.y).max().unwrap();

      for y in min_y..=max_y {
         for x in min_x..=max_x {
            let point = Point { x, y };

            if point == (Point { x: 0, y: 0 }) {
               write!(f, "s")?;
            } else if point == self.head {
               write!(f, "H")?;
            } else if self.tail.contains(&point) {
               write!(f, "T")?;
            } else if self.visited.contains(&point) {
               write!(f, "#")?;
            } else {
               write!(f, ".")?;
            }
         }
         writeln!(f)?;
      }

      Ok(())
   }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
   Up,
   Down,
   Left,
   Right,
}

#[derive(Debug, Clone, Copy)]
struct Motion {
   direction: Direction,
   distance: usize,
}

fn parse_input() -> Vec<Motion> {
   include_str!("../input/day09.txt")
      .lines()
      .map(|line| line.split(' ').collect::<Vec<_>>())
      .map(|tokens| Motion {
         direction: match tokens[0] {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => unreachable!(),
         },
         distance: tokens[1].parse().unwrap(),
      })
      .collect()
}

fn simulate(motion: Motion, state: &mut SimulationState, debug: bool) {
   let (dx, dy) = match motion.direction {
      Direction::Up => (0, -1),
      Direction::Down => (0, 1),
      Direction::Left => (-1, 0),
      Direction::Right => (1, 0),
   };

   for _ in 0..motion.distance {
      // Move the head.
      state.head.x += dx;
      state.head.y += dy;

      let mut previous_knot = state.head;

      // Move the tail.
      for knot in state.tail.iter_mut() {
         if (knot.x - previous_knot.x).abs() == 2 || (knot.y - previous_knot.y).abs() == 2 {
            knot.x += (previous_knot.x - knot.x).signum();
            knot.y += (previous_knot.y - knot.y).signum();
         }

         // Invariant: After moving a knot, it is never more than 1 unit away
         // from the knot (previous_knot) in front of it.
         assert!((knot.x - previous_knot.x).abs() <= 1 && (knot.y - previous_knot.y).abs() <= 1);

         previous_knot = *knot;
      }

      state.visited.insert(*state.tail.last().unwrap());

      if debug {
         print!("{}[2J", 27 as char);
         println!("{}", state);
         println!("Moved in direction: {:?}", motion.direction);
         sleep(Duration::from_millis(50));
      }
   }
}

pub fn solve_part_one() -> usize {
   let motions = parse_input();
   let mut state = SimulationState::new(2);

   for motion in motions {
      simulate(motion, &mut state, false);
   }

   state.visited.len()
}

pub fn solve_part_two() -> usize {
   let motions = parse_input();
   let mut state = SimulationState::new(10);

   for motion in motions {
      simulate(motion, &mut state, false);
   }

   state.visited.len()
}
