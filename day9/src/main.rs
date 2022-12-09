use std::collections::HashSet;
use std::io;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Point {
  x : i32,
  y : i32,
}

fn update_tail_pos(head_pos : Point, tail_pos : Point) -> Point {
  let x_dist = head_pos.x - tail_pos.x;
  let y_dist = head_pos.y - tail_pos.y;

  // Can only have moved in one direction, and initial position must have
  // been adjacent.
  assert!(x_dist.abs() <= 1 || y_dist.abs() <= 1);

  let mut new_tail_pos = tail_pos;

  if x_dist > 1 {
    new_tail_pos.x = tail_pos.x + x_dist.abs() - 1;
    new_tail_pos.y = head_pos.y;
  } else if x_dist < -1 {
    new_tail_pos.x = tail_pos.x - x_dist.abs() + 1;
    new_tail_pos.y = head_pos.y;
  }

  if y_dist > 1 {
    new_tail_pos.y = new_tail_pos.y + y_dist.abs() - 1;
    new_tail_pos.x = head_pos.x;
  } else if y_dist < -1 {
    new_tail_pos.y = new_tail_pos.y - y_dist.abs() + 1;
    new_tail_pos.x = head_pos.x;
  }

  new_tail_pos
}

fn parse_moves(lines : impl Iterator<Item = io::Result<String>>) -> HashSet<Point> {

  let mut positions = HashSet::new();

  // Start at origin x=0, y=0.
  let mut head_pos = Point { x : 0, y : 0 };
  let mut tail_pos = Point { x : 0, y : 0 };

  for line in lines {
    let line = line.unwrap();
    let parts : Vec<&str> = line.split_whitespace().collect();
    let direction = parts[0];
    let distance = parts[1].parse::<u32>().unwrap();

    // Process each move in increments of one.
    for _ in 0..distance {
      head_pos = match direction {
        "U" => Point { x : head_pos.x, y : head_pos.y + 1 },
        "D" => Point { x : head_pos.x, y : head_pos.y - 1 },
        "L" => Point { x : head_pos.x - 1, y : head_pos.y },
        "R" => Point { x : head_pos.x + 1, y : head_pos.y },
        _ => panic!("unexpected direction token {}", direction),
      };

      tail_pos = update_tail_pos(head_pos, tail_pos);
      positions.insert(tail_pos);

      #[cfg(debug_assertions)]
      println!("head:{:?}, tail:{:?}", head_pos, tail_pos);
    }
  }

  positions
}

fn main() {
  let tail_positions = parse_moves(io::stdin().lines());

  #[cfg(debug_assertions)]
  println!("{:?}", tail_positions);

  println!("part 1: {}", tail_positions.len());
}
