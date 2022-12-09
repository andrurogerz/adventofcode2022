use std::collections::HashSet;
use std::io;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Point {
  x : i32,
  y : i32,
}

fn update_tail_pos(head_pos : Point, tail_pos : Point) -> Point {
  let dx = head_pos.x - tail_pos.x;
  let dy = head_pos.y - tail_pos.y;

  assert!(dx.abs() <= 2 && dy.abs() <= 2);

  let mut new_tail_pos = tail_pos;

  if dx > 1 {
    new_tail_pos.x = tail_pos.x + dx.abs() - 1;
  } else if dx < -1 {
    new_tail_pos.x = tail_pos.x - dx.abs() + 1;
  } else if dy.abs() > 1 {
    new_tail_pos.x = head_pos.x;
  }

  if dy > 1 {
    new_tail_pos.y = new_tail_pos.y + dy.abs() - 1;
  } else if dy < -1 {
    new_tail_pos.y = new_tail_pos.y - dy.abs() + 1;
  } else if dx.abs() > 1 {
    new_tail_pos.y = head_pos.y
  }

  new_tail_pos
}

fn parse_moves(lines : impl Iterator<Item = String>, knot_count : usize) -> HashSet<Point> {
  assert!(knot_count > 1);

  // Start each knot at origin x=0, y=0.
  let mut knots = Vec::new();
  for _ in 0..knot_count {
    knots.push(Point { x : 0, y : 0 });
  }

  let mut tail_positions = HashSet::new();

  for line in lines {
    let parts : Vec<&str> = line.split_whitespace().collect();
    let direction = parts[0];
    let distance = parts[1].parse::<u32>().unwrap();

    // Process each move in increments of one.
    for _ in 0..distance {
      knots[0] = match direction {
        "U" => Point { x : knots[0].x, y : knots[0].y + 1 },
        "D" => Point { x : knots[0].x, y : knots[0].y - 1 },
        "L" => Point { x : knots[0].x - 1, y : knots[0].y },
        "R" => Point { x : knots[0].x + 1, y : knots[0].y },
        _ => panic!("unexpected direction token {}", direction),
      };

      // Update each trailing knot after every move.
      for i in 1..knot_count {
        knots[i] = update_tail_pos(knots[i-1], knots[i]);
      }

      // Keep track of positions visited by the last knot only.
      tail_positions.insert(knots[knot_count - 1]);
    }

    #[cfg(debug_assertions)]
    println!("{:?}", knots);
  }

  tail_positions
}

fn main() {
  let mut stdin : Vec<String> = Vec::new();
  for line in io::stdin().lines() {
    stdin.push(line.unwrap().to_string());
  }

  let tail_positions = parse_moves(stdin.clone().into_iter(), 2);

  #[cfg(debug_assertions)]
  println!("{:?}", tail_positions);
  println!("part 1: {}", tail_positions.len());

  let tail_positions = parse_moves(stdin.clone().into_iter(), 10);

  #[cfg(debug_assertions)]
  println!("{:?}", tail_positions);
  println!("part 2: {}", tail_positions.len());
}
