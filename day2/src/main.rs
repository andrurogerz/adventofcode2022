use std::io;

fn calculate_score(lines : impl Iterator<Item = io::Result<String>>) -> u64 {
  let mut total_score : u64 = 0;
  for line in lines {
    let chars : Vec<char> = line.unwrap().chars().collect();
    // Assume only well-structure input.
    assert_eq!(chars.len(), 3);
    assert_eq!(chars[1], ' ');
    let round_score = match (chars[0], chars[2]) {
      ('A', 'X') => { 1 + 3 }, // them:rock, me:rock => 4
      ('A', 'Y') => { 2 + 6 }, // them:rock, me:paper => 8
      ('A', 'Z') => { 3 + 0 }, // them:rock, me:scissors => 3
      ('B', 'X') => { 1 + 0 }, // them:paper, me:rock => 1
      ('B', 'Y') => { 2 + 3 }, // them:paper, me:paper => 5
      ('B', 'Z') => { 3 + 6 }, // them:paper, me:scissors => 9
      ('C', 'X') => { 1 + 6 }, // them:scissors, me:rock => 7
      ('C', 'Y') => { 2 + 0 }, // them:scissors, me:paper => 2
      ('C', 'Z') => { 3 + 3 }, // them:scissors, me:scissors => 6
      (a, b) => {
        // Assume only well-structure input.
        panic!("invalid input: ({}, {})", a, b);
      }
    };
    total_score += round_score;
  }
  total_score
}

fn main() {
  let score = calculate_score(io::stdin().lines());
  println!("part 1: {}", score);
}
