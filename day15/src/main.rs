use std::collections::HashMap;
use std::error::Error;
use std::io;

#[derive(Hash, Clone, Copy, Debug, Eq, PartialEq)]
struct Position {
  x: i32,
  y: i32,
}

fn parse_input(lines: &mut impl Iterator<Item = io::Result<String>>)
    -> Result<HashMap<Position, Position>, Box<dyn Error>> {
  let mut positions = HashMap::new();

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

    assert!(!positions.contains_key(&Position { x: sensor_x, y: sensor_y }));
    assert!(!positions.contains_key(&Position { x: beacon_x, y: beacon_y }));

    positions.insert(
      Position { x: sensor_x, y: sensor_y },
      Position { x: beacon_x, y: beacon_y });
  }

  Ok(positions)
}

fn main() -> Result<(), Box<dyn Error>> {
  let positions = parse_input(&mut io::stdin().lines())?;

  #[cfg(debug_assertions)]
  println!("{:?}", positions);

  Ok(())
}
