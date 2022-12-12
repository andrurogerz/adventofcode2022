use std::cmp::min;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io;

const START : char = 'S';
const END : char = 'E';

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Square {
  x : usize,
  y : usize,
}

fn parse_topography(lines : &mut impl Iterator<Item = io::Result<String>>) -> (Square, Vec<Square>, Vec<Vec<char>>) {
  let mut map : Vec<Vec<char>> = Vec::new();
  let mut start = Square { x : usize::MAX, y : usize::MAX };
  let mut potential_start_squares : Vec<Square> = Vec::new();
  let mut end = Square { x : usize::MAX, y : usize::MAX };

  let mut y = 0;
  for line in lines {
    let row : Vec<char> = line.unwrap().chars().collect();
    for x in 0..row.len() {
      let ch = row[x];
      assert!((ch as u8 >= 'a' as u8 && ch as u8 <= 'z' as u8) || ch == START || ch == END);

      if ch == START {
        start = Square { x, y };
      } else if ch == END {
        end = Square { x, y };
      } else if ch == 'a' {
        potential_start_squares.push(Square { x, y });
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

  (start, potential_start_squares, map)
}

fn height_of_square(map : &Vec<Vec<char>>, square : Square) -> usize {
  assert!(square.y < map.len());
  assert!(square.x < map[0].len());
  match map[square.y][square.x] {
    START => 'a' as usize,
    END => 'z' as usize,
    _ => map[square.y][square.x] as usize,
  }
}

fn find_shortest_path_len(map : &Vec<Vec<char>>, start_squares : Vec<Square>) -> usize {
  assert!(!map.is_empty());
  assert!(!map[0].is_empty());

  let row_count = map.len();
  let col_count = map[0].len();

  #[cfg(debug_assertions)]
  println!("cols:{}, rows:{}", col_count, row_count);

  // Treat the map as an tree where every node is a reachable square on the map.
  // Each node's children is the set of squares reachable from it that have not
  // already been seen. We perform a breadth-first search of the tree to find
  // the shortest path from start to end. Use a queue to track possible moves,
  // where each item in the queue is a tuple of a square locaiton and the
  // number of moves it took to reach it (which is the least possible for that
  // square).
  let mut possible_moves : VecDeque<(usize, Square)> = VecDeque::new();
  for square in start_squares {
    assert_eq!(height_of_square(&map, square), 'a' as usize);
    possible_moves.push_back((0usize, square));
  }

  // The set of squares already visited so we don't bother visiting them a
  // second time. If a square was reachable in N moves, there is no point in
  // visiting it on any move > N even if from a different path. This avoids
  // cycles and lets us traverse as a tree rather than a graph.
  let mut visited_squares : HashSet<Square> = HashSet::new();

  while !possible_moves.is_empty() {
    let (move_count, current_square) = possible_moves.pop_front().unwrap();

    // Have we reached the end?
    if map[current_square.y][current_square.x] == END {
      return move_count;
    }

    // Explore possible moves from the current square.
    let mut adjacent_squares = Vec::with_capacity(4);

    // Left
    if current_square.x > 0 {
      adjacent_squares.push(Square { x : current_square.x - 1, y : current_square.y });
    }

    // Right
    if current_square.x < col_count - 1 {
      adjacent_squares.push(Square { x : current_square.x + 1, y : current_square.y });
    }

    // Up
    if current_square.y > 0 {
      adjacent_squares.push(Square { x : current_square.x, y : current_square.y - 1 });
    }

    // Down
    if current_square.y < row_count - 1 {
      adjacent_squares.push(Square { x : current_square.x, y : current_square.y + 1 });
    }

    #[cfg(debug_assertions)]
    println!("moves from {:?}:", current_square);

    for next_square in adjacent_squares {
      if visited_squares.contains(&next_square) {
        #[cfg(debug_assertions)]
        println!("    skip {:?} (visited)", next_square);
        continue;
      }

      if height_of_square(map, next_square) <= height_of_square(map, current_square) + 1 {
        #[cfg(debug_assertions)]
        println!("    candidate {:?}", next_square);
        possible_moves.push_back((move_count + 1, next_square));
        visited_squares.insert(next_square);

      } else {
        #[cfg(debug_assertions)]
        println!("    skip {:?} (too high)", next_square);
      }
    }
  }

  unreachable!();
}

fn main() {
  let (start, potential_start_squares, map) = parse_topography(&mut io::stdin().lines());

  #[cfg(debug_assertions)]
  for row in &map {
    println!("{:?}", row);
  }

  let part_1 = find_shortest_path_len(&map, vec![start]);
  println!("part 1: {}", part_1);

  let part_2 = find_shortest_path_len(&map, potential_start_squares);
  println!("part 2: {}", min(part_1, part_2));
}
