use std::cmp;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::io;

#[derive(Hash, Clone, Copy, Debug, Eq, PartialEq)]
struct Position {
  x: i32,
  y: i32,
}

impl Position {
  fn distance_from(&self, other: &Position) -> i32 {
    let dx: i32 = self.x - other.x;
    let dy: i32 = self.y - other.y;
    dx.abs() + dy.abs()
  }
}

#[derive(Clone)]
struct Grid {
  min: Position,
  max: Position,
  positions: HashMap<Position, Position>,
}

impl Grid {
  fn insert(&mut self, sensor_pos: Position, beacon_pos: Position) {
    let dist = sensor_pos.distance_from(&beacon_pos);

    // Update grid bounds.
    self.min.x = cmp::min(self.min.x, sensor_pos.x - dist);
    self.min.y = cmp::min(self.min.y, sensor_pos.y - dist);
    self.max.x = cmp::max(self.max.x, sensor_pos.x + dist);
    self.max.y = cmp::max(self.max.y, sensor_pos.y + dist);

    self.positions.insert(sensor_pos, beacon_pos);
  }

  fn count_definitely_not_beacons_in_row(&self, row: i32) -> usize {
    let mut count = 0usize;
    for x in self.min.x..=self.max.x {
      let current_pos = Position { x, y: row };
      for (&sensor_pos, &beacon_pos) in &self.positions {
        let max_dist = sensor_pos.distance_from(&beacon_pos);
        let actual_dist = sensor_pos.distance_from(&current_pos);
        if current_pos == beacon_pos {
          // Already a beacon at this position.
          break;

        } else if current_pos == sensor_pos {
          // A sensor in this position, so can't be a beacon.
          count += 1;
          break;

        } else if actual_dist <= max_dist {
          // Within manhattan distance of the sensor, so can't be a beacon.
          count += 1;
          break;
        }
        // Else, this position could possibly be a beacon.
      }
    }

    count
  }

  fn find_possible_beacons_in_range(&self, min: Position, max: Position) -> Vec<Position> {
    assert!(min.x <= max.x);
    assert!(min.y <= max.y);

    let mut possible_beacons = Vec::new();

    for y in min.y..=max.y {
      let mut x = min.x;
      while x <= max.x {
        let mut next_x = x;
        for (&sensor_pos, &beacon_pos) in &self.positions {
          let max_sensor_range = sensor_pos.distance_from(&beacon_pos);
          let dx: i32 = sensor_pos.x - x;
          let dy: i32 = sensor_pos.y - y;
          if dx.abs() + dy.abs() <= max_sensor_range {
            // This point is within the range of the sensor.
            next_x = cmp::max(next_x, 1 + sensor_pos.x + max_sensor_range - dy.abs());
          }
        }

        if x == next_x {
          // Found a point not within range of a sensor.
          possible_beacons.push(Position { x, y });
          break;
        }

        x = next_x;
      }
    }

    possible_beacons
  }
}

impl fmt::Debug for Grid {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    for y in self.min.y..=self.max.y {
      write!(f, "{0:4}: ", y)?;
      for x in self.min.x..=self.max.x {
        let current_pos = Position { x, y };
        let mut ch = '?';
        for (&sensor_pos, &beacon_pos) in &self.positions {
          let max_dist = sensor_pos.distance_from(&beacon_pos);
          let actual_dist = sensor_pos.distance_from(&current_pos);
          if current_pos == beacon_pos {
            ch = 'B';
            break;
          } else if current_pos == sensor_pos {
            ch = 'S';
            break;
          } else if actual_dist <= max_dist {
            ch = '#';
            break;
          }
        }

        write!(f, "{}", ch)?;
      }
      writeln!(f, "")?;
    }

    fmt::Result::Ok(())
  }
}

fn parse_input(lines: &mut impl Iterator<Item = io::Result<String>>)
    -> Result<Grid, Box<dyn Error>> {
  let mut grid = Grid {
     min: Position { x: i32::MAX, y: i32::MAX },
     max: Position { x: i32::MIN, y: i32::MIN },
     positions: HashMap::new(),
  };

  for line in lines {
    let line = line?;

    // Parse sensor X position.
    let text = "Sensor at x=";
    let tok_start = text.len();
    assert_eq!(text, &line[..tok_start]);
    let tok_end = tok_start + line[tok_start..].find(',').unwrap();
    let sensor_x = line[tok_start..tok_end].parse::<i32>()?;

    // Parse sensor Y position.
    let text = " y=";
    let tok_start = tok_end + 1 + text.len();
    assert_eq!(text, &line[tok_end + 1..tok_end + 1 + text.len()]);
    let tok_end = tok_start + line[tok_start..].find(':').unwrap();
    let sensor_y = line[tok_start..tok_end].parse::<i32>()?;

    // Parse beacon X position.
    let text = " closest beacon is at x=";
    let tok_start = tok_end + 1 + text.len();
    assert_eq!(text, &line[tok_end + 1..tok_end + 1 + text.len()]);
    let tok_end = tok_start + line[tok_start..].find(',').unwrap();
    let beacon_x = line[tok_start..tok_end].parse::<i32>()?;

    // Parse beacon Y position.
    let text = " y=";
    let tok_start = tok_end + 1 + text.len();
    assert_eq!(text, &line[tok_end + 1..tok_end + 1 + text.len()]);
    let beacon_y= line[tok_start..].parse::<i32>()?;

    assert!(!grid.positions.contains_key(&Position { x: sensor_x, y: sensor_y }));
    assert!(!grid.positions.contains_key(&Position { x: beacon_x, y: beacon_y }));

    grid.insert(
      Position { x: sensor_x, y: sensor_y },
      Position { x: beacon_x, y: beacon_y });
  }

  Ok(grid)
}

fn main() -> Result<(), Box<dyn Error>> {
  let grid = parse_input(&mut io::stdin().lines())?;

  #[cfg(debug_assertions)]
  println!("{:?}", grid);

  const TEST_ROW : i32 = 2000000;
  let result = grid.count_definitely_not_beacons_in_row(TEST_ROW);
  println!("part 1: {}", result);

  const MIN: Position = Position { x: 0, y: 0 };
  const MAX: Position = Position { x: 4000000, y: 4000000 };
  let possible_beacons = grid.find_possible_beacons_in_range(MIN, MAX);

  #[cfg(debug_assertions)]
  println!("{:?}", possible_beacons);

  assert_eq!(1, possible_beacons.len());

  #[cfg(debug_assertions)]
  for (&sensor_pos, &beacon_pos) in &grid.positions {
    let max_dist = sensor_pos.distance_from(&beacon_pos);
    let actual_dist = sensor_pos.distance_from(&possible_beacons[0]);
    assert!(actual_dist > max_dist);
  }

  let x = possible_beacons[0].x as usize;
  let y = possible_beacons[0].y as usize;
  let result = x * 4000000 + y;
  println!("part 2: {}", result);

  Ok(())
}
