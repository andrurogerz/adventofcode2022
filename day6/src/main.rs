use std::io;

fn find_start_of_packet_index(packet : &String) -> usize {
  let packet_chars : Vec<char> = packet.chars().collect();
  for i in 0..packet.len() {
    if packet_chars[i + 0] != packet_chars[i + 1] &&
       packet_chars[i + 0] != packet_chars[i + 2] &&
       packet_chars[i + 0] != packet_chars[i + 3] &&
       packet_chars[i + 1] != packet_chars[i + 2] &&
       packet_chars[i + 1] != packet_chars[i + 3] &&
       packet_chars[i + 2] != packet_chars[i + 3] {
      return i + 4;
    }
  }
  unreachable!();
}

fn find_start_of_message_index(packet : &String) -> usize {
  const PACKET_LEN : usize = 14usize;
  let packet_chars : Vec<char> = packet.chars().collect();
  for i in 0..packet.len() {
    // A bit array to track which characters have been seen.
    let mut seen_chars : [bool; std::char::MAX as usize] = [false; std::char::MAX as usize];

    for j in 0..PACKET_LEN {
      // Must never run off the end of valid input.
      assert!(i + j < packet_chars.len());

      let ch = packet_chars[i + j] as usize;

      if seen_chars[ch] {
        // Already seen this character in the range we're looking at.
        break;
      }

      seen_chars[ch] = true;

      if j + 1 == PACKET_LEN {
        return i + j + 1;
      }
    }
  }

  // Never reachable with valid input.
  unreachable!();
}

fn main() {
  let mut input = String::new();
  io::stdin().read_line(&mut input).unwrap();
  let result = find_start_of_packet_index(&input);
  println!("part 1: {}", result);

  let result = find_start_of_message_index(&input);
  println!("part 2: {}", result);
}
