use std::cmp;
use std::io;

fn pretty_print_stacks(stacks : &Vec<Vec<char>>) {
  // Determine the deepest stack for an upper bound on printing.
  let mut max_rows = 0;
  for stack in stacks {
    max_rows = cmp::max(max_rows, stack.len());
  }

  for row_idx in (0..max_rows).rev() {
    for col_idx in 0..stacks.len() {
      assert!(stacks[col_idx].len() <= max_rows);
      if row_idx < stacks[col_idx].len() {
        print!("[{}] ", stacks[col_idx][row_idx]);
      } else {
        print!("    ");
      }
    }
    println!("");
  }
}

fn parse_stacks(lines : &mut impl Iterator<Item = io::Result<String>>) -> Vec<Vec<char>> {
  // Consume lines until we hit a blank line, indicating the end of the stack
  // diagram portion of the input.
  let mut stack_lines = Vec::new();
  for line in lines {
    let line = line.unwrap();
    if line.len() == 0 {
      break;
    }
    stack_lines.push(line);
  }

  // Last line of the stack diagram tells us how many stacks there are.
  let line = stack_lines.pop().unwrap();
  let parts : Vec<&str> = line.split_whitespace().collect();
  let stack_count = parts.len();

  let mut stacks = Vec::with_capacity(stack_count);
  for _i in 0..stack_count {
    stacks.push(Vec::new());
  }

  // Convert stack lines into a vector of stacks.
  const CHARS_PER_ITEM : usize = 4usize;
  for line in stack_lines.iter().rev() {
    let chars : Vec<char> = line.chars().collect();
    for i in (0..chars.len()).step_by(CHARS_PER_ITEM) {
      let item : char = chars[i + 1];
      if item == ' ' {
        continue;
      }

      stacks[i / CHARS_PER_ITEM].push(item);
    }
  }

  stacks
}

fn execute_actions(stacks : &mut Vec<Vec<char>>, lines : &mut impl Iterator<Item = io::Result<String>>) {
  for line in lines {
    let line = line.unwrap();
    let parts : Vec<&str> = line.split_whitespace().collect();
    assert_eq!(parts.len(), 6);

    let (count, source_idx, dest_idx)  = (
        parts[1].parse::<usize>().unwrap(),
        parts[3].parse::<usize>().unwrap() - 1,
        parts[5].parse::<usize>().unwrap() - 1);
    assert!(source_idx < stacks.len());
    assert!(dest_idx < stacks.len());

    let mut tmp_stack = Vec::with_capacity(count);
    for _i in 0..count {
      let item = stacks[source_idx].pop().unwrap();
      tmp_stack.push(item);
    }

    for _i in 0..count {
      let item = tmp_stack.pop().unwrap();
      stacks[dest_idx].push(item);
    }
  }
}

fn main() {
  let mut lines = io::stdin().lines();
  let mut stacks = parse_stacks(&mut lines);

  println!("Start state:");
  pretty_print_stacks(&stacks);

  execute_actions(&mut stacks, &mut lines);

  println!("End state:");
  pretty_print_stacks(&stacks);

  let mut result = String::new();

  for stack in stacks {
    result.push(*stack.last().unwrap());
  }

  println!("part 2: {}", result);
}
