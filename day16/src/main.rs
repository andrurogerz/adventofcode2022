use std::cmp;
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

// Reduce the graph by "inlining" all nodes with flow rate of zero and removing
// them from the graph. Edge costs are updated to reflect the cost of traversing
// "through" the zero flow rate nodes. The start node is left intact even if it
// has a flow rate of zero.
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
        if next_valve.flow_rate == 0 && next_valve_id != start_valve_id {
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

// Find the shortest path between two nodes in the graph.
fn find_shortest_path(start_id: ValveId, end_id: ValveId, graph: &ValveGraph, visited: &HashSet<ValveId>) -> usize {
  assert!(graph.contains_key(&start_id));
  assert!(graph.contains_key(&end_id));
  assert!(!visited.contains(&start_id));
  assert!(!visited.contains(&end_id));

  if start_id == end_id {
    return 0;
  }

  let mut visited = visited.clone();
  visited.insert(start_id);

  let valve = graph.get(&start_id).unwrap();
  let mut dist = usize::MAX;
  for (&next_id, next_dist) in &valve.adjacent_valves {
    if visited.contains(&next_id) {
      continue;
    }

    let result = find_shortest_path(next_id, end_id, graph, &visited);
    if result == usize::MAX {
      continue;
    }

    dist = cmp::min(dist, next_dist + result);
  }

  dist
}

// Create a copy of the input graph where every node is connected to every other
// node with an edge reflecting the cost to traverse to it.
fn connected_graph_create(valve_graph: &ValveGraph) -> ValveGraph {
  let mut connected_graph = HashMap::new();

  // Build a fully connected graph.
  let valve_ids: Vec<ValveId> = valve_graph.keys().cloned().collect();
  for i in 0..valve_ids.len() {
    let mut valve_edges = HashMap::new();
    for j in 0..valve_ids.len() {
      if i == j {
        continue;
      }

      valve_edges.insert(
          valve_ids[j],
          find_shortest_path(valve_ids[i], valve_ids[j], valve_graph, &HashSet::new()));
    }

    let valve = valve_graph.get(&valve_ids[i]).unwrap();
    connected_graph.insert(
        valve_ids[i],
        Valve { flow_rate: valve.flow_rate, adjacent_valves: valve_edges });
  }

  connected_graph
}

// Calculate the max pressure that can be released by releasing remaining valves
// in the optimal order given the remaining time.
fn find_max_pressure_released(valve_id: ValveId, connected_graph: &ValveGraph, time_remaining: usize) -> usize {
  assert!(time_remaining > 0);

  let mut pressure_released = 0usize;
  let current_valve = connected_graph.get(&valve_id).unwrap();

  // Create an updated version of the connected graph with this valve removed
  // so it doesn't get visited again.
  let mut updated_graph = connected_graph.clone();
  updated_graph.remove(&valve_id);

  // Try traversing to every remaining node in the graph and determine which one
  // will give us the max pressure released. The updated graph may now be empty,
  // in which case we've hit the base case and will make no more recursive calls.
  for &next_valve_id in updated_graph.keys() {
    let time_cost = current_valve.adjacent_valves.get(&next_valve_id).unwrap();

    if time_cost + 1 >= time_remaining {
      // Not enough time to traverse to and release this valve.
      continue;
    }

    // Calculate time remaining if we traverse to this valve and release it.
    let next_time_remaining = time_remaining - time_cost - 1;

    // Recursively calculate the pressure released from the next valve and the
    // remaining graph.
    pressure_released = cmp::max(pressure_released,
        find_max_pressure_released(next_valve_id, &updated_graph, next_time_remaining));
  }

  pressure_released + (current_valve.flow_rate * time_remaining)
}

fn main() -> Result<(), Box<dyn Error>> {
  let mut valve_graph = parse_input(&mut io::stdin().lines())?;

  println!("complete graph:");
  print_valve_graph(&valve_graph);

  let start_valve_id = ['A', 'A'];
  reduce_graph(start_valve_id, &mut valve_graph);

  println!("reduced graph:");
  print_valve_graph(&valve_graph);

  let connected_graph = connected_graph_create(&valve_graph);

  println!("connected graph:");
  print_valve_graph(&connected_graph);

  let result = find_max_pressure_released(start_valve_id, &connected_graph, 30);
  println!("part 1: {}", result);

  Ok(())
}
