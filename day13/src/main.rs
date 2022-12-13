use std::cmp::Ordering;
use std::io;

#[derive(Clone, Debug, Eq, PartialEq)]
enum Packet {
  Value(usize),
  ValueList(Vec<Packet>),
}

impl Ord for Packet {
  fn cmp(&self, other: &Self) -> Ordering {
    match (self, other) {

      (Packet::Value(left), Packet::Value(right)) => {
        if left < right {
          return Ordering::Less;

        } else if right < left {
          return Ordering::Greater;

        } else {
          return Ordering::Equal;
        }
      },

      (Packet::ValueList(left), Packet::ValueList(right)) => {
        for i in 0usize..usize::MAX {
          if i >= left.len() || i >= right.len() {
            break;
          }

          let cmp = left[i].cmp(&right[i]);
          if cmp == Ordering::Equal {
            continue;
          }

          return cmp;
        }

        if left.len() > right.len() {
          return Ordering::Greater;
        } else if right.len() > left.len() {
          return Ordering::Less;
        }

        return Ordering::Equal;
      },

      (Packet::Value(_), Packet::ValueList(_)) => {
        return Packet::ValueList(vec![self.clone()]).cmp(other);
      },

      (Packet::ValueList(_), Packet::Value(_)) => {
        return self.cmp(&Packet::ValueList(vec![other.clone()]));
      },
    }
  }
}

impl PartialOrd for Packet {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

fn parse_packet_recursive(packet_desc : &[char]) -> (usize, Packet) {
  assert!(!packet_desc.is_empty());

  let mut packets : Vec<Packet> = Vec::new();
  assert_eq!(packet_desc[0], '[');
  let mut i = 1usize;
  while i < packet_desc.len() {
    match packet_desc[i] {
      '[' => {
        let (consumed, packet) = parse_packet_recursive(&packet_desc[i..]);
        packets.push(packet);
        i += consumed;
      },
      ']' => {
        // Packet end.
        return (i + 1, Packet::ValueList(packets));
      }
      ',' => {
        i += 1;
      }
      _ => {
        assert!(packet_desc[i].is_ascii_digit());
        let mut val = 0usize;
        while packet_desc[i].is_ascii_digit() {
          val = (val * 10) + packet_desc[i] as usize - '0' as usize;
          i += 1;
        }
        assert!(packet_desc[i] == ',' || packet_desc[i] == ']');
        packets.push(Packet::Value(val));
      }
    }
  }

  unreachable!();
}

fn parse_packet(packet_desc : &str) -> Packet {
  let chars : Vec<char> = packet_desc.chars().collect();
  let (consumed, packet) = parse_packet_recursive(&chars);
  assert_eq!(consumed, packet_desc.len());

  packet
}

fn parse_input(lines : &mut impl Iterator<Item = io::Result<String>>) -> Vec<(Packet, Packet)> {
  let mut packet_pairs = Vec::new();

  loop {
    packet_pairs.push((
      match lines.next() {
        Some(Ok(left)) => parse_packet(&left),
        Some(Err(err)) => panic!("fatal error {}", err),
        None => panic!("malformed input"),
      },
      match lines.next() {
        Some(Ok(right)) => parse_packet(&right),
        Some(Err(err)) => panic!("fatal error {}", err),
        None => panic!("malformed input"),
      }
    ));

    match lines.next() {
      Some(Ok(val)) => assert!(val.is_empty()),
      Some(Err(err)) => panic!("fatal error {}", err),
      None => break, // Done parsing input.
    }
  }

  packet_pairs
}

fn sum_ordered_packet_indices(packet_pairs : &Vec<(Packet, Packet)>) -> usize {
  let mut result = 0usize;
  let mut pair_index = 1usize;
  for (left_packet, right_packet) in packet_pairs {
    #[cfg(debug_assertions)]
    println!("packet pair {}:\n  {:?}\n  {:?}", pair_index, left_packet, right_packet);

    if left_packet <= right_packet {
      #[cfg(debug_assertions)]
      println!("packets {} are ordered", pair_index);
      result += pair_index;
    }

    pair_index += 1;
  }

  result
}

fn main() {
  let packet_pairs = parse_input(&mut io::stdin().lines());
  let result = sum_ordered_packet_indices(&packet_pairs);
  println!("part 1: {}", result);
}
