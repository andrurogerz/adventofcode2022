use std::collections::HashMap;
use std::error::Error;
use std::io;

type ValveId = [char; 2];

#[derive(Debug)]
struct Valve {
  flow_rate: usize,
  adjacent_valves: Vec<ValveId>,
}

type ValveMap = HashMap<ValveId, Valve>;

fn parse_input(lines: &mut impl Iterator<Item = io::Result<String>>)
    -> Result<ValveMap, Box<dyn Error>> {
  let mut valve_map = HashMap::new();

  for line in lines {
    let line = line?;

    // Parse valve ID.
    let text = "Valve ";
    let tok_start = text.len();
    assert_eq!(text, &line[..tok_start]);
    let tok_end = tok_start + line[tok_start..].find(' ').unwrap();
    let mut id_iter = line[tok_start..tok_end].chars();
    let valve_id : ValveId = [id_iter.next().unwrap(), id_iter.next().unwrap()];
    assert_eq!(id_iter.next(), None); // only two chars.

    // Parse flow rate.
    let text = "has flow rate=";
    let tok_start = tok_end + 1 + text.len();
    assert_eq!(text, &line[tok_end + 1..tok_end + 1 + text.len()]);
    let tok_end = tok_start + line[tok_start..].find(';').unwrap();
    let flow_rate = line[tok_start..tok_end].parse::<usize>()?;

    // Parse to the adjacent valves list. Text is different for singlar...
    let tok_start = {
      let text = " tunnels lead to valves ";
      let tok_start = tok_end + 1 + text.len();
      if text == &line[tok_end + 1..tok_end + 1 + text.len()] {
        tok_start
      } else {
        let text = " tunnel leads to valve ";
        let tok_start = tok_end + 1 + text.len();
        assert_eq!(text, &line[tok_end + 1..tok_end + 1 + text.len()]);
        tok_start
      }
    };

    // Parse the adjacent valves list.
    let mut adjacent_valves = Vec::new();
    let mut id_iter = line[tok_start..].chars();
    loop {
      let valve_id : ValveId = [id_iter.next().unwrap(), id_iter.next().unwrap()];
      let n = id_iter.next();
      match n {
        None => {
          adjacent_valves.push(valve_id);
          break;
        },
        Some(',') => {
          adjacent_valves.push(valve_id);
          // Advance the iterator one more past the space.
          let n = id_iter.next();
          match n {
            None => panic!("unexpected end of input"),
            Some(' ') => (),
            Some(c) => panic!("unexpected input character {}", c),
          };
        },
        Some(c) => panic!("unexpected input character {}", c),
      }
    }

    #[cfg(debug_assertions)]
    println!("Valve:{}, flow_rate:{}, tunnels to:{:?}",
      String::from_iter(valve_id), flow_rate, adjacent_valves);

    assert!(!adjacent_valves.is_empty());
    valve_map.insert(valve_id, Valve { flow_rate, adjacent_valves });
  }

  Ok(valve_map)
}

fn main() -> Result<(), Box<dyn Error>> {
  let valve_map = parse_input(&mut io::stdin().lines())?;

  Ok(())
}
