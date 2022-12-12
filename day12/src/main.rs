use std::collections::HashSet;
use std::collections::VecDeque;
use std::io;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Square {
  x : usize,
  y : usize,
}

fn parse_topography(lines : &mut impl Iterator<Item = io::Result<String>>) -> (Square, Vec<Vec<char>>) {
  let mut map : Vec<Vec<char>> = Vec::new();
  let mut start = Square { x : usize::MAX, y : usize::MAX };
  let mut end = Square { x : 0, y : 0 };

  let mut y = 0;
  for line in lines {
    let row : Vec<char> = line.unwrap().chars().collect();
    for x in 0..row.len() {
      let ch = row[x];
      assert!((ch as u8 >= 'a' as u8 && ch as u8 <= 'z' as u8) || ch == 'S' || ch == 'E');
      if ch == 'S' {
        start = Square { x, y };
      } else if ch == 'E' {
        end = Square { x, y };
      }
    }

    #[cfg(debug_assertions)]
    if map.len() > 0 {
      assert_eq!(row.len(), map[0].len());
    }

    map.push(row);
    y += 1;
  }

  // Make sure we found start and end points
  assert!(start.x != usize::MAX);
  assert!(end.x != usize::MAX);

  (start, map)
}

fn find_shortest_path_len(map : &Vec<Vec<char>>, start : Square) -> usize {
  assert!(!map.is_empty());
  assert!(!map[0].is_empty());

  let row_count = map.len();
  let col_count = map[0].len();

  #[cfg(debug_assertions)]
  println!("cols:{}, rows:{}", col_count, row_count);

  let mut debug_steps = usize::MAX;

  // Treat the map as an tree where every node is a reachable square on the map.
  // Each node's children is the set of squares reachable from it that have not
  // already been seen. We perform a breadth-first search of the tree to find
  // the shortest path from start to end. Use a queue to track possible moves,
  // where each item in the queue is a tuple of a square locaiton and the
  // number of moves it took to reach it (which is the least possible for that
  // square).
  let mut possible_moves : VecDeque<(usize, Square)> = VecDeque::new();
  possible_moves.push_back((0usize, start));

  // The set of squares already visited so we don't bother visiting them a
  // second time. If a square was reachable in N moves, there is no point in
  // visiting it on any move > N even if from a different path. This avoids
  // cycles and lets us traverse as a tree rather than a graph.
  let mut visited_squares : HashSet<Square> = HashSet::new();

  while !possible_moves.is_empty() {
    let (count, dest) = possible_moves.pop_front().unwrap();

    // Have we reached the end?
    if map[dest.y][dest.x] == 'E' {
      return count;
    }

    // Explore possible moves from destination
    let mut adjacent_squares = Vec::with_capacity(4);

    if dest.x > 0 {
      adjacent_squares.push(Square { x : dest.x - 1, y : dest.y });
    }

    if dest.x < col_count - 1 {
      adjacent_squares.push(Square { x : dest.x + 1, y : dest.y });
    }

    if dest.y > 0 {
      adjacent_squares.push(Square { x : dest.x, y : dest.y - 1 });
    }

    if dest.y < row_count - 1 {
      adjacent_squares.push(Square { x : dest.x, y : dest.y + 1 });
    }

    let dest_height = match map[dest.y][dest.x] {
      'S' => 'a' as usize,
      'E' => 'z' as usize,
      _ => map[dest.y][dest.x] as usize,
    };

    #[cfg(debug_assertions)]
    println!("moves from {:?}:", dest);

    for next in adjacent_squares {
      if visited_squares.contains(&next) {
        #[cfg(debug_assertions)]
        println!("    skip {:?} (visited)", next);
        continue;
      }

      let next_height = match map[next.y][next.x] {
        'S' => 'a' as usize,
        'E' => 'z' as usize,
        _ => map[next.y][next.x] as usize,
      };

      if next_height <= dest_height + 1 {
        #[cfg(debug_assertions)]
        println!("    candidate {:?}", next);
        possible_moves.push_back((count + 1, next));
        visited_squares.insert(next);

      } else {
        #[cfg(debug_assertions)]
        println!("    skip {:?} (too high)", next);
      }
    }

    debug_steps -= 1;
    if debug_steps == 0 {
      break;
    }
  }

  unreachable!();
}

fn main() {
  let (start, map) = parse_topography(&mut io::stdin().lines());

  #[cfg(debug_assertions)]
  for row in &map {
    println!("{:?}", row);
  }

  let result = find_shortest_path_len(&map, start);
  println!("part 1: {}", result);
}
