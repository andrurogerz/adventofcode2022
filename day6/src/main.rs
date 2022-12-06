use std::io;

fn find_start_of_packet_index(packet : String) -> usize {
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

fn main() {
  let mut input = String::new();
  io::stdin().read_line(&mut input).unwrap();
  let result = find_start_of_packet_index(input);
  println!("part 1: {}", result);
}
