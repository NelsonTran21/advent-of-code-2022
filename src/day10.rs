use std::{collections::VecDeque, fmt::Display};

#[derive(Debug, Clone, Copy)]
enum Instruction {
   AddX(i32),
   NoOp,
}

type Program = VecDeque<Instruction>;

#[derive(Debug, Clone, Copy)]
struct ExecutingInstruction {
   instruction: Instruction,
   cycles_left: i32,
}

impl From<Instruction> for ExecutingInstruction {
   fn from(instruction: Instruction) -> Self {
      match instruction {
         Instruction::AddX(_) => ExecutingInstruction {
            instruction,
            cycles_left: 2,
         },
         Instruction::NoOp => ExecutingInstruction {
            instruction,
            cycles_left: 1,
         },
      }
   }
}

struct CPU {
   program: Option<Program>,
   queue: Option<ExecutingInstruction>,
   current_cycle: i32,
   signal_strengths: Vec<i32>,
   x_register: i32,
}

impl CPU {
   fn new() -> Self {
      CPU {
         program: None,
         queue: None,
         current_cycle: 0,
         signal_strengths: Vec::new(),
         x_register: 1,
      }
   }

   fn load_program(&mut self, program: &Program) {
      self.program = Some(program.clone());
   }

   fn execute_cycle(&mut self, debug: bool) {
      match self.program {
         None => panic!("No program loaded"),

         Some(ref mut program) => {
            self.current_cycle += 1;

            if (self.current_cycle - 20) % 40 == 0 {
               let signal_strength = self.x_register * self.current_cycle;
               self.signal_strengths.push(signal_strength);
            }

            if debug {
               print!("{}: ", self.current_cycle);
            }

            if self.queue.is_none() {
               let next_instruction = program.pop_front();

               if let Some(next_instruction) = next_instruction {
                  self.queue = Some(next_instruction.into());

                  if debug {
                     print!("Queued {:?}. ", next_instruction);
                  }
               }
            }

            if let Some(ref mut queue) = self.queue {
               queue.cycles_left -= 1;

               if queue.cycles_left == 0 {
                  match queue.instruction {
                     Instruction::AddX(value) => self.x_register += value,
                     Instruction::NoOp => (),
                  }

                  if debug {
                     print!("Executed {:?}. ", queue.instruction);
                  }

                  self.queue = None;
               }
            }

            if debug {
               println!("(X={})", self.x_register);
            }
         }
      }
   }

   fn is_finished(&self) -> bool {
      match self.program {
         None => panic!("No program loaded"),
         Some(ref program) => program.is_empty(),
      }
   }
}

struct CRT {
   pixels: [[bool; 40]; 6],
}

impl CRT {
   fn new() -> Self {
      CRT {
         pixels: [[false; 40]; 6],
      }
   }

   fn draw_pixel(&mut self, cycle: i32, sprite_position: i32) {
      let y = (cycle / 40) as usize;
      let x = (cycle % 40) as usize;

      let sprite_cursor = sprite_position - 1..=sprite_position + 1;
      if sprite_cursor.contains(&(x as i32)) {
         self.pixels[y][x] = true;
      }
   }
}

impl Display for CRT {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      for row in self.pixels.iter() {
         for pixel in row.iter() {
            let character = if *pixel { '#' } else { '.' };
            write!(f, "{}", character)?;
         }
         writeln!(f)?;
      }
      Ok(())
   }
}

fn parse_input() -> Program {
   include_str!("../input/day10.txt")
      .lines()
      .map(|line| {
         let tokens = line.split(" ").collect::<Vec<_>>();
         let instruction = tokens.get(0);
         let value = tokens.get(1).and_then(|token| token.parse::<i32>().ok());

         match instruction {
            Some(&"addx") => Instruction::AddX(value.unwrap()),
            Some(&"noop") => Instruction::NoOp,
            _ => unreachable!(),
         }
      })
      .collect()
}

pub fn solve_part_one() -> i32 {
   let program = parse_input();
   let mut cpu = CPU::new();

   cpu.load_program(&program);
   while !cpu.is_finished() {
      cpu.execute_cycle(false);
   }

   cpu.signal_strengths.iter().sum()
}

pub fn solve_part_two() -> String {
   let program = parse_input();
   let mut cpu = CPU::new();
   let mut crt = CRT::new();

   cpu.load_program(&program);
   while !cpu.is_finished() {
      crt.draw_pixel(cpu.current_cycle, cpu.x_register);
      cpu.execute_cycle(false);
   }

   crt.to_string()
}
