#[derive(Debug, Copy, Clone, PartialEq)]
enum Shape {
   Rock = 1,
   Paper = 2,
   Scissors = 3,
}

#[derive(Debug, Copy, Clone)]
enum Outcome {
   Loss = 0,
   Draw = 3,
   Win = 6,
}

#[derive(Debug, Copy, Clone)]
struct Game(Shape, Shape);

#[derive(Debug, Clone, Copy)]
struct AltGame(Shape, Outcome);

type Score = u32;

fn get_outcome(game: Game) -> Outcome {
   match game {
      Game(Shape::Rock, Shape::Rock) => Outcome::Draw,
      Game(Shape::Rock, Shape::Paper) => Outcome::Win,
      Game(Shape::Rock, Shape::Scissors) => Outcome::Loss,
      Game(Shape::Paper, Shape::Rock) => Outcome::Loss,
      Game(Shape::Paper, Shape::Paper) => Outcome::Draw,
      Game(Shape::Paper, Shape::Scissors) => Outcome::Win,
      Game(Shape::Scissors, Shape::Rock) => Outcome::Win,
      Game(Shape::Scissors, Shape::Paper) => Outcome::Loss,
      Game(Shape::Scissors, Shape::Scissors) => Outcome::Draw,
   }
}

fn evaluate(game: Game) -> Score {
   (game.1 as Score) + (get_outcome(game) as Score)
}

fn convert_game(game: AltGame) -> Game {
   let player_move = match game {
      AltGame(Shape::Rock, Outcome::Loss) => Shape::Scissors,
      AltGame(Shape::Rock, Outcome::Draw) => Shape::Rock,
      AltGame(Shape::Rock, Outcome::Win) => Shape::Paper,
      AltGame(Shape::Paper, Outcome::Loss) => Shape::Rock,
      AltGame(Shape::Paper, Outcome::Draw) => Shape::Paper,
      AltGame(Shape::Paper, Outcome::Win) => Shape::Scissors,
      AltGame(Shape::Scissors, Outcome::Loss) => Shape::Paper,
      AltGame(Shape::Scissors, Outcome::Draw) => Shape::Scissors,
      AltGame(Shape::Scissors, Outcome::Win) => Shape::Rock,
   };

   Game(game.0, player_move)
}

fn parse_move(letter: &'static str) -> Shape {
   match letter {
      "A" => Shape::Rock,
      "B" => Shape::Paper,
      "C" => Shape::Scissors,
      "X" => Shape::Rock,
      "Y" => Shape::Paper,
      "Z" => Shape::Scissors,
      _ => unreachable!(),
   }
}

fn parse_outcome(letter: &'static str) -> Outcome {
   match letter {
      "X" => Outcome::Loss,
      "Y" => Outcome::Draw,
      "Z" => Outcome::Win,
      _ => unreachable!(),
   }
}

fn parse_part_one_input() -> Vec<Game> {
   include_str!("../input/day02.txt")
      .lines()
      .map(|line| {
         let moves: Vec<&'static str> = line.split(" ").collect();
         Game(parse_move(moves[0]), parse_move(moves[1]))
      })
      .collect()
}

fn parse_part_two_input() -> Vec<AltGame> {
   include_str!("../input/day02.iq.txt")
      .lines()
      .map(|line| {
         let moves: Vec<&'static str> = line.split(" ").collect();
         AltGame(parse_move(moves[0]), parse_outcome(moves[1]))
      })
      .collect()
}

pub fn solve_part_one() -> Score {
   let games = parse_part_one_input();
   games.into_iter().map(evaluate).sum()
}

pub fn solve_part_two() -> Score {
   let games = parse_part_two_input();
   games.into_iter().map(convert_game).map(evaluate).sum()
}
