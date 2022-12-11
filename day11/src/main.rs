use std::io;

use usize as Item;

#[derive(Debug)]
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

#[derive(Debug)]
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

fn execute_round(monkeys : &mut Vec<Monkey>) {

  for i in 0..monkeys.len() {
    let monkey = &mut monkeys[i];
    let mut throws = Vec::new();

    while !monkey.items.is_empty() {
      monkey.inspection_count += 1;
      let old_item = monkey.items.pop().unwrap();
      let new_item = monkey.op.apply(old_item) / 3;

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
  let mut first_place = &monkeys[0];
  let mut second_place = &monkeys[1];
  for i in 2..monkeys.len() {
    let monkey = &monkeys[i];
    if monkey.inspection_count > first_place.inspection_count {
      second_place = first_place;
      first_place = &monkey;
    } else if monkey.inspection_count > second_place.inspection_count {
      second_place = &monkey;
    }
  }

  #[cfg(debug_assertions)]
  println!("first place: {:?}\nsecond_place: {:?}", first_place, second_place);

  first_place.inspection_count * second_place.inspection_count
}

fn main() {
  let mut monkeys = parse_input(&mut io::stdin().lines());

  #[cfg(debug_assertions)]
  {
    println!("starting state:");
    for monkey in &monkeys {
      println!("{:?}", monkey);
    }
  }

  for _ in 1..=20 {
    execute_round(&mut monkeys);

    #[cfg(debug_assertions)]
    {
      println!("end of round state:");
      for monkey in &monkeys {
        println!("{:?}", monkey);
      }
    }
  }

  let result = calculate_monkey_business(&monkeys);
  println!("part 1: {}", result);
}
