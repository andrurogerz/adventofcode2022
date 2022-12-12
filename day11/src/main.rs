use std::io;

use usize as Item;

#[derive(Debug, Clone)]
enum Operation {
  Add(usize),
  Mult(usize),
  Pow(usize),
}

impl Operation {
  fn apply(&self, arg : usize) -> usize {
    match *self {
      Self::Add(val) => arg + val,
      Self::Mult(val) => arg * val,
      Self::Pow(val) => arg.pow(val.try_into().unwrap()),
    }
  }
}

#[derive(Debug, Clone)]
struct Monkey {
  inspection_count : usize,
  items : Vec<usize>,
  op : Operation,
  test_divisor : usize,
  if_true_target : usize,
  if_false_target : usize,
}

fn parse_items(items_line : &str) -> Vec<Item> {
  let mut items = Vec::new();

  let parts : Vec<&str> = items_line.split(':').collect();
  assert_eq!(parts.len(), 2);
  assert_eq!(parts[0].trim(), "Starting items");
  let parts : Vec<&str> = parts[1].split(',').collect();
  for part in parts {
    items.push(part.trim().parse::<usize>().unwrap());
  }

  items
}

fn parse_operation(operation_line : &str) -> Operation {
  let parts : Vec<&str> = operation_line.split(':').collect();
  assert_eq!(parts.len(), 2);
  assert_eq!(parts[0].trim(), "Operation");

  let parts : Vec<&str> = parts[1].split_whitespace().collect();
  assert_eq!(parts.len(), 5);
  assert_eq!(parts[0], "new");
  assert_eq!(parts[1], "=");
  assert_eq!(parts[2], "old");

  match parts[3] {
    "+" => Operation::Add(parts[4].parse::<usize>().unwrap()),
    "*" => {
      match parts[4] {
        "old" => Operation::Pow(2),
        _ => Operation::Mult(parts[4].parse::<usize>().unwrap()),
      }
    },
    _ => panic!("unexpected operation: {}", parts[2]),
  }
}

fn parse_test(test_line : &str) -> usize {
  let parts : Vec<&str> = test_line.split(':').collect();
  assert_eq!(parts[0].trim(), "Test");
  assert_eq!(parts.len(), 2);

  let parts : Vec<&str> = parts[1].split_whitespace().collect();
  assert_eq!(parts.len(), 3);
  assert_eq!(parts[0], "divisible");
  assert_eq!(parts[1], "by");

  parts[2].parse::<usize>().unwrap()
}

fn parse_target(target_line : &str) -> usize {
  let parts : Vec<&str> = target_line.split_whitespace().collect();
  assert_eq!(parts.len(), 4);
  assert_eq!(parts[0], "throw");
  assert_eq!(parts[1], "to");
  assert_eq!(parts[2], "monkey");

  parts[3].parse::<usize>().unwrap()
}

fn parse_monkey(lines : &mut impl Iterator<Item = io::Result<String>>) -> Monkey {
  let items = parse_items(&lines.next().unwrap().unwrap());
  let op = parse_operation(&lines.next().unwrap().unwrap());
  let test_divisor = parse_test(&lines.next().unwrap().unwrap());

  let if_true_line = lines.next().unwrap().unwrap();
  let parts : Vec<&str> = if_true_line.split(':').collect();
  assert_eq!(parts.len(), 2);
  assert_eq!(parts[0].trim(), "If true");
  let if_true_target = parse_target(parts[1].trim());

  let if_false_line = lines.next().unwrap().unwrap();
  let parts : Vec<&str> = if_false_line.split(':').collect();
  assert_eq!(parts.len(), 2);
  assert_eq!(parts[0].trim(), "If false");
  let if_false_target = parse_target(parts[1].trim());

  Monkey { inspection_count : 0, items, op, test_divisor, if_true_target, if_false_target }
}

fn parse_input(lines : &mut impl Iterator<Item = io::Result<String>>) -> Vec<Monkey> {
  let mut monkeys = Vec::new();

  let mut next_line = lines.next();
  while next_line.is_some() {

    let line = next_line.unwrap().unwrap();
    assert!(line.starts_with("Monkey"));
    assert!(line.ends_with(":"));

    let monkey = parse_monkey(lines);
    monkeys.push(monkey);

    // Skip newline between monkey descriptors.
    let newline = lines.next();
    if newline.is_some() {
      assert_eq!(newline.unwrap().unwrap().len(), 0);
    }

    next_line = lines.next();
  }

  monkeys
}

fn execute_round(monkeys : &mut Vec<Monkey>, lcm : usize, worry_divisor : usize) {

  for i in 0..monkeys.len() {
    let monkey = &mut monkeys[i];
    let mut throws = Vec::new();

    while !monkey.items.is_empty() {
      monkey.inspection_count += 1;
      let old_item = monkey.items.pop().unwrap();
      let new_item = (monkey.op.apply(old_item) / worry_divisor) % lcm;

      let target_monkey;
      if new_item % monkey.test_divisor == 0 {
        target_monkey = monkey.if_true_target;

      } else {
        target_monkey = monkey.if_false_target;
      }

      assert!(target_monkey != i);

      throws.push((target_monkey, new_item));
    }

    // Apply throw operations.
    while !throws.is_empty() {
      let throw = throws.pop().unwrap();
      monkeys[throw.0].items.push(throw.1);
    }
  }
}

fn calculate_monkey_business(monkeys : &Vec<Monkey>) -> usize {
  assert!(monkeys.len() > 1);

  let mut first_place = 0usize;
  let mut second_place = 0usize;

  for monkey in monkeys {
    let value = monkey.inspection_count;
    if value > first_place {
      second_place = first_place;
      first_place = value;
    } else if value > second_place {
      second_place = value;
    }
  }

  #[cfg(debug_assertions)]
  println!("first place: {}\nsecond_place: {}", first_place, second_place);

  first_place * second_place
}

fn calculate_lcm(values : &Vec<usize>) -> usize {
  assert!(!values.is_empty());

  // Brute-force least common multiple calculation.
  for i in 1..usize::MAX {
    let lcm_candidate = values[0] * i;
    for j in 1..values.len() {
      if lcm_candidate % values[j] != 0 {
        break;
      }

      if j + 1 == values.len() {
        #[cfg(debug_assertions)]
        println!("found lcm: {}", lcm_candidate);
        return lcm_candidate;
      }
    }
  }
  unreachable!();
}

fn execute_rounds(input : &Vec<Monkey>, round_count : usize, worry_divisor : usize) -> usize {
  let mut monkeys = input.clone();

  // Find the least common multiple of all the divisors so we can keep our worry
  // level from growing exponentially.
  let mut divisors = Vec::with_capacity(monkeys.len() + 1);
  for monkey in &monkeys {
    divisors.push(monkey.test_divisor);
  }
  divisors.push(worry_divisor);
  let lcm = calculate_lcm(&divisors);

  #[cfg(debug_assertions)]
  {
    println!("starting state:");
    for monkey in &monkeys {
      println!("{:?}", monkey);
    }
  }

  for _ in 1..=round_count {
    execute_round(&mut monkeys, lcm, worry_divisor);

    #[cfg(debug_assertions)]
    {
      println!("end of round state:");
      for monkey in &monkeys {
        println!("{:?}", monkey);
      }
    }
  }

  calculate_monkey_business(&monkeys)
}

fn main() {
  let input = parse_input(&mut io::stdin().lines());

  let result = execute_rounds(&input, 20, 3);
  println!("part 1: {}", result);

  let result = execute_rounds(&input, 10000, 1);
  println!("part 2: {}", result);
}
