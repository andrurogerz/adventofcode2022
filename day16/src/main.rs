use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::error::Error;
use std::io;

type ValveId = [char; 2];

#[derive(Debug, Clone)]
struct Valve {
  flow_rate: usize,
  adjacent_valves: HashMap<ValveId, usize>,
}

type ValveGraph = HashMap<ValveId, Valve>;

fn print_valve_graph(valve_graph: &ValveGraph) {
  for (valve_id, valve) in valve_graph {
    print!("Valve:{}, flow_rate:{}, tunnels to:",
      String::from_iter(valve_id), valve.flow_rate);
    for (valve_id, distance) in &valve.adjacent_valves  {
      print!("{}{}({}), ", valve_id[0], valve_id[1], distance);
    }
    println!("");
  }
}

fn parse_input(lines: &mut impl Iterator<Item = io::Result<String>>)
    -> Result<ValveGraph, Box<dyn Error>> {
  let mut valve_graph = HashMap::new();

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
    let mut adjacent_valves = HashMap::new();
    let mut id_iter = line[tok_start..].chars();
    loop {
      let valve_id : ValveId = [id_iter.next().unwrap(), id_iter.next().unwrap()];
      let n = id_iter.next();
      match n {
        None => {
          adjacent_valves.insert(valve_id, 1);
          break;
        },
        Some(',') => {
          adjacent_valves.insert(valve_id, 1);

          // Advance the iterator past the space char.
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

    assert!(!adjacent_valves.is_empty());
    valve_graph.insert(valve_id, Valve { flow_rate, adjacent_valves });
  }

  Ok(valve_graph)
}

fn reduce_graph(start_valve_id : ValveId, valve_graph: &mut ValveGraph) {
  let mut valves_to_remove : HashSet<ValveId> = HashSet::new();

  loop {
    let mut inline_count = 0;
    let mut node_queue : VecDeque<ValveId> = VecDeque::new();
    node_queue.push_back(start_valve_id);

    let mut visited_nodes : HashSet<ValveId> = HashSet::new();
    while !node_queue.is_empty() {
      let valve_id = node_queue.pop_front().unwrap();
      if visited_nodes.contains(&valve_id) {
        continue;
      }
      visited_nodes.insert(valve_id);

      let valve = valve_graph.get(&valve_id).unwrap();

      // Build a new list of adjacent values.
      let mut updated_adjacent_valves : HashMap<ValveId, usize> = HashMap::new();
      for (&next_valve_id, &next_distance) in &valve.adjacent_valves {
        assert!(next_valve_id != valve_id);
        let next_valve = valve_graph.get(&next_valve_id).unwrap();
        if next_valve.flow_rate == 0 {
          // Inline the path from valve to next_valve.
          for (&next_next_valve_id, next_next_distance) in &next_valve.adjacent_valves {
            if next_next_valve_id != valve_id &&
               !valve.adjacent_valves.contains_key(&next_next_valve_id) {
              // Don't link valve to itself or to valves it is already linked to.
              updated_adjacent_valves.insert(next_next_valve_id, next_distance + next_next_distance);
            }
          }

          // Track all zero flow valves to remove from the graph on completion.
          if next_valve_id != start_valve_id {
            valves_to_remove.insert(next_valve_id);
          }

          inline_count += 1;

        } else {
          // Keep existing path.
          updated_adjacent_valves.insert(next_valve_id, next_distance);
        }
        node_queue.push_back(next_valve_id);
      }

      for &next_valve_id in updated_adjacent_valves.keys() {
        node_queue.push_back(next_valve_id);
      }

      let valve = valve_graph.get_mut(&valve_id).unwrap();
      valve.adjacent_valves = updated_adjacent_valves;
    }

    if inline_count == 0 {
      // There were no more graph updates on this last iteration.
      break;
    }
  }

  // Now remove any zero flow rate valves from the graph (other than start)
  // since there are no longer direct paths to it.
  for valve_id in &valves_to_remove {
    valve_graph.remove(valve_id);
  }

  #[cfg(debug_assertions)]
  for valve_id in valve_graph.keys() {
    let valve = valve_graph.get(valve_id).unwrap();
    // Make sure every edge is connected to something valid in the graph.
    for valve_id in valve.adjacent_valves.keys() {
      assert!(valve_graph.contains_key(valve_id));
    }
  }
}

fn main() -> Result<(), Box<dyn Error>> {
  let mut valve_graph = parse_input(&mut io::stdin().lines())?;

  println!("complete graph:");
  print_valve_graph(&valve_graph);

  let start_valve_id = ['A', 'A'];
  reduce_graph(start_valve_id, &mut valve_graph);

  println!("reduced graph:");
  print_valve_graph(&valve_graph);

  Ok(())
}
