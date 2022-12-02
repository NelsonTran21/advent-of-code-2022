enum Part {
   One,
   Two,
}

#[derive(Copy, Clone, PartialEq)]
enum Shape {
   Rock = 1,
   Paper = 2,
   Scissors = 3,
}

#[derive(Copy, Clone)]
enum Outcome {
   Loss = 0,
   Draw = 3,
   Win = 6,
}

type Score = u32;

#[derive(Copy, Clone)]
struct Game(Shape, Shape);

#[derive(Copy, Clone)]
struct Strategy(Shape, Outcome);

impl Into<Game> for Strategy {
   fn into(self) -> Game {
      use Outcome::*;
      use Shape::*;

      let player_shape = match self {
         Strategy(Rock, Loss) => Scissors,
         Strategy(Rock, Draw) => Rock,
         Strategy(Rock, Win) => Paper,

         Strategy(Paper, Loss) => Rock,
         Strategy(Paper, Draw) => Paper,
         Strategy(Paper, Win) => Scissors,

         Strategy(Scissors, Loss) => Paper,
         Strategy(Scissors, Draw) => Scissors,
         Strategy(Scissors, Win) => Rock,
      };

      Game(self.0, player_shape)
   }
}

fn parse_shape(letter: &'static str) -> Shape {
   use Shape::*;

   match letter {
      "A" | "X" => Rock,
      "B" | "Y" => Paper,
      "C" | "Z" => Scissors,

      _ => unreachable!(),
   }
}

fn parse_outcome(letter: &'static str) -> Outcome {
   use Outcome::*;

   match letter {
      "X" => Loss,
      "Y" => Draw,
      "Z" => Win,

      _ => unreachable!(),
   }
}

fn parse_input(part: Part) -> Vec<Game> {
   include_str!("../input/day02.txt")
      .lines()
      .map(|line| {
         let tokens: Vec<&'static str> = line.split(" ").collect();
         let opponent = parse_shape(tokens[0]);
         let player = parse_shape(tokens[1]);
         let outcome = parse_outcome(tokens[1]);

         match part {
            Part::One => Game(opponent, player),
            Part::Two => Strategy(opponent, outcome).into(),
         }
      })
      .collect()
}

fn get_outcome(game: Game) -> Outcome {
   use Outcome::*;
   use Shape::*;

   match game {
      Game(Rock, Rock) => Draw,
      Game(Rock, Paper) => Win,
      Game(Rock, Scissors) => Loss,

      Game(Paper, Rock) => Loss,
      Game(Paper, Paper) => Draw,
      Game(Paper, Scissors) => Win,

      Game(Scissors, Rock) => Win,
      Game(Scissors, Paper) => Loss,
      Game(Scissors, Scissors) => Draw,
   }
}

fn evaluate(game: Game) -> Score {
   (game.1 as Score) + (get_outcome(game) as Score)
}

pub fn solve_part_one() -> Score {
   parse_input(Part::One).into_iter().map(evaluate).sum()
}

pub fn solve_part_two() -> Score {
   parse_input(Part::Two).into_iter().map(evaluate).sum()
}
