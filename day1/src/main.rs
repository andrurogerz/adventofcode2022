use std::io;

fn group_sums(lines : impl Iterator<Item = io::Result<String>>) -> Vec<u64> {
  let mut sums : Vec<u64> = Vec::new();
  let mut cur_sum : u64 = 0;
  for line in lines {
    match line.unwrap().parse::<u64>() {
      Ok(val) => {
        cur_sum += val;
      }
      Err(_) => {
        // We are assuming only well-structured input, so integer parse errors
        // indicate line break between groups of integers.
        sums.push(cur_sum);
        cur_sum = 0;
      }
    }
  }
  sums.sort();
  sums.reverse();
  sums
}

fn main() {
  let sums = group_sums(io::stdin().lines());
  println!("part 1: {}", sums[0]);
  println!("part 2: {}", sums[0] + sums[1] + sums[2]);
}
