type Crate = char;

type Stack = Vec<Crate>;

#[derive(Copy, Clone)]
struct Move {
   count: usize,
   from: usize,
   to: usize,
}

fn parse_input() -> (Vec<Stack>, Vec<Move>) {
   let mut input_lines = include_str!("../input/day05.txt")
      .lines()
      .collect::<Vec<_>>()
      .into_iter();

   // Consume until we hit the x-axis.
   let diagram_text: Vec<&str> = input_lines
      .by_ref()
      .take_while(|line| !line.starts_with(" 1"))
      .collect();

   // Count the number of stacks depicted in the diagram.
   // Each crate takes up 3 characters (i.e. "[N]"), and
   // they are delimited by spaces.
   let stack_count = diagram_text[0].len() / 4 + 1;
   let mut stacks: Vec<Stack> = vec![vec![]; stack_count];

   // Parse the crates and add them to the stacks.
   for line in &diagram_text {
      for stack_index in 0..stack_count {
         if let Some(value @ 'A'..='Z') = line.chars().nth(stack_index * 4 + 1) {
            stacks[stack_index].push(value);
         }
      }
   }

   // Reverse the stacks because we pushed the crates in reverse order.
   stacks.iter_mut().for_each(|stack| stack.reverse());

   // Consume the next empty line in the input.
   input_lines.next();

   // Parse the list of moves.
   let moves: Vec<Move> = input_lines
      .map(|line| {
         let tokens: Vec<_> = line.split(" ").collect();

         Move {
            count: tokens[1].parse().unwrap(),
            from: tokens[3].parse::<usize>().unwrap() - 1,
            to: tokens[5].parse::<usize>().unwrap() - 1,
         }
      })
      .collect();

   (stacks, moves)
}

// lol, what are the chances that both "move" and "crate"
// are reserved words in Rust? ¯\_(ツ)_/¯
fn apply_move(stacks: &mut Vec<Stack>, _move: &Move) {
   for _ in 1..=_move.count {
      let _crate = stacks[_move.from].pop().unwrap();
      stacks[_move.to].push(_crate);
   }
}

fn apply_move_9001(stacks: &mut Vec<Stack>, _move: &Move) {
   let from = &mut stacks[_move.from];
   let crates = from.split_off(from.len() - _move.count);
   stacks[_move.to].extend(crates);
}

pub fn solve_part_one() -> String {
   let (mut stacks, moves) = parse_input();

   for _move in moves {
      apply_move(&mut stacks, &_move);
   }

   String::from_iter(
      stacks
         .into_iter()
         .map(|stack| stack.last().unwrap().clone()),
   )
}

pub fn solve_part_two() -> String {
   let (mut stacks, moves) = parse_input();

   for _move in moves {
      apply_move_9001(&mut stacks, &_move);
   }

   String::from_iter(
      stacks
         .into_iter()
         .map(|stack| stack.last().unwrap().clone()),
   )
}
