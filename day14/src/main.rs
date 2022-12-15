use std::cmp;
use std::collections::HashSet;
use std::error::Error;
use std::fmt;
use std::io;

#[derive(Hash, Clone, Copy, Debug, Eq, PartialEq)]
struct Position {
  x : usize,
  y : usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Contents {
  Empty,
  Rock,
  Sand,
}

struct Grid(Vec<Vec<Contents>>);

// Debug pretty-print for a grid.
impl fmt::Debug for Grid {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

    // Find the min non-empty X to start printing.
    let mut min_x = usize::MAX;
    let mut max_x = 0usize;
    for y in 0..self.0.len() {
      for x in 0..self.0[0].len() {
        min_x = match self.0[y][x] {
          Contents::Empty => min_x,
          _ => cmp::min(min_x, x),
        };
        max_x = match self.0[y][x] {
          Contents::Empty => max_x,
          _ => cmp::max(max_x, x),
        };
      }
    }

    // Print every row starting from min X.
    for y in 0..self.0.len() {
      for x in min_x..=max_x {
        write!(f, "{}", match self.0[y][x] {
          Contents::Empty => '.',
          Contents::Rock => '#',
          Contents::Sand => 'o',
        })?;
      }
      writeln!(f, "")?;
    }

    fmt::Result::Ok(())
  }
}

fn parse_input(lines : &mut impl Iterator<Item = io::Result<String>>)
    -> Result<Grid, Box<dyn Error>> {

  let mut rocks : HashSet<Position> = HashSet::new();
  let mut max_x = 0;
  let mut max_y = 0;

  for line in lines {
    let line = line?;
    let mut points : Vec<Position> = Vec::new();
    let point_strs = line.split(" -> ");
    for point in point_strs {
      let coords : Vec<&str> = point.split(',').collect();
      assert_eq!(coords.len(), 2);
      points.push(
        Position {
          x : coords[0].parse::<usize>()?,
          y : coords[1].parse::<usize>()?,
        });
    }

    assert!(points.len() > 1); // no lines from a single point

    let mut pos_prev = &points[0];
    for i in 1..points.len() {
      let pos_cur = &points[i];
      let dx = pos_cur.x as i64 - pos_prev.x as i64;
      let dy = pos_cur.y as i64 - pos_prev.y as i64;

      // no diagonal lines
      assert!((dx == 0) ^ (dy == 0));

      if dx != 0 {
        // Horizontal line.
        let x_start = cmp::min(pos_prev.x, pos_cur.x);
        let x_end = cmp::max(pos_prev.x, pos_cur.x);
        let y = pos_prev.y;
        for x in x_start..=x_end {
          let pos = Position { x, y };
          rocks.insert(pos);
          max_x = cmp::max(max_x, x);
        }
      }

      if dy != 0 {
        // Vertical line.
        let x = pos_prev.x;
        let y_start = cmp::min(pos_prev.y, pos_cur.y);
        let y_end = cmp::max(pos_prev.y, pos_cur.y);
        for y in y_start..=y_end {
          let pos = Position { x, y };
          rocks.insert(pos);
          max_y = cmp::max(max_y, y);
        }
      }

      pos_prev = pos_cur;
    }
  }

  let mut grid = Grid(Vec::new());
  for y in 0..=max_y {

    let mut row = Vec::with_capacity(max_x);
    for x in 0..=(max_x * 2) {
      if rocks.contains(&Position { x, y }) {
        row.push(Contents::Rock);
      } else {
        row.push(Contents::Empty);
      }
    }
    grid.0.push(row);
  }

  Ok(grid)
}

fn execute(grid : &mut Grid, sand_start : Position) -> usize {
  assert!(grid.0.len() > 0);
  assert!(grid.0[0].len() > 0);

  let mut sand_unit_count = 0usize;
  'outer: loop {

    let mut sand_pos = sand_start;
    'inner: loop {

      let below_pos = Position { x : sand_pos.x, y : sand_pos.y + 1 };
      if below_pos.y >= grid.0.len() {
        break 'outer;
      }

      let next_sand_pos = match grid.0[below_pos.y][below_pos.x] {
        Contents::Empty => below_pos,
        _ => {
          match grid.0[below_pos.y][below_pos.x - 1] {
            Contents::Empty => Position { x : below_pos.x - 1, y : below_pos.y },
            _ => {
              match grid.0[below_pos.y][below_pos.x + 1] {
                Contents::Empty => Position { x : below_pos.x + 1, y : below_pos.y },
                _ => sand_pos, // doesn't move
              }
            }
          }
        }
      };

      if next_sand_pos == sand_pos {
        // Sand didn't move, so we're done with this one.
        break 'inner;
      }

      sand_pos = next_sand_pos;
    }

    // Update the grid with sand contents.
    sand_unit_count += 1;
    grid.0[sand_pos.y][sand_pos.x] = Contents::Sand;
  }

  #[cfg(debug_assertions)]
  println!("{:?}", grid);

  sand_unit_count
}

fn main() -> Result<(), Box<dyn Error>> {
  let mut grid = parse_input(&mut io::stdin().lines())?;

  #[cfg(debug_assertions)]
  println!("{:?}", grid);

  let result = execute(&mut grid, Position { x : 500, y : 0 });
  println!("part 1: {}", result);

  Ok(())
}
