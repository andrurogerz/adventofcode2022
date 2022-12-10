use std::io;

#[derive(Debug)]
enum Instr {
  Noop,
  AddX(i64),
}

impl Instr {
  fn cycle_count(&self) -> usize {
    match *self {
      Self::Noop => 1,
      Self::AddX(_) => 2,
    }
  }
}

fn parse_input(lines : impl Iterator<Item = io::Result<String>>) -> Vec<Instr> {
  let mut instrs = Vec::new();

  for line in lines {
    let line = line.unwrap();
    let parts : Vec<&str> = line.split_whitespace().collect();
    assert!(parts.len() > 0);
    assert!(parts.len() <= 2);
    instrs.push(
        match parts[0] {
          "noop" => Instr::Noop,
          "addx" => Instr::AddX(parts[1].parse::<i64>().unwrap()),
          _ => panic!("illegal instruction {}", parts[1]),
        }
      );
  }

  instrs
}

fn execute(instrs : Vec<Instr>) -> (i64, String) {
  const DISPLAY_COLS : usize = 40usize;
  const DISPLAY_ROWS : usize = 7usize;

  let mut total_signal_strength = 0i64;
  let mut display = String::with_capacity((DISPLAY_COLS + 1) * DISPLAY_ROWS);

  let mut pc = 0;
  let mut reg_x = 1i64;
  let mut instr = &Instr::Noop;
  let mut cycle_counter = 0usize;
  let mut x_pos = 0usize;

  for cycle in 1..usize::MAX {

    // Update signal strength at start of cycle.
    if (cycle == 20) || (cycle > 20 && (cycle - 20) % 40 == 0) {
      // Update signal value.
      let signal_strength = reg_x * cycle as i64;
      total_signal_strength += signal_strength;

      #[cfg(debug_assertions)]
      println!("signal strength: {} {} {}", cycle, reg_x, signal_strength);
    }

    let pixel = if (x_pos as i64 >= reg_x - 1) && (x_pos as i64 <= reg_x + 1) { '#' } else { '.' };
    display.push(pixel);
    x_pos = (x_pos + 1) % DISPLAY_COLS;
    if x_pos == 0 {
      display.push('\n');
    }

    if cycle_counter == 0 {
      // Fetch instruction.
      instr = &instrs[pc];
      cycle_counter = instr.cycle_count();
    }

    cycle_counter -= 1;
    if cycle_counter > 0 {
      // Stall.
      continue;
    }

    // Execute current instruction when cycle counter reaches 0.
    match instr {
      Instr::Noop => {},
      Instr::AddX(val) => {
        reg_x += val;
      },
    }

    // Move to next instruction.
    pc = pc + 1;
    if !(pc < instrs.len()) {
      // Reached end of instruction sequence.
      break;
    }
  }

  (total_signal_strength, display)
}

fn main() {
  let instrs = parse_input(io::stdin().lines());
  let (total_signal_strength, display) = execute(instrs);
  println!("part 1: {}", total_signal_strength);
  println!("part 2:\n{}", display);
}
