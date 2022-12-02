use std::io;

fn max_group_sum(lines : impl Iterator<Item = io::Result<String>>) -> u64 {
  let mut max_sum : u64 = 0;
  let mut cur_sum : u64 = 0;
  for line in lines {
    match line.unwrap().parse::<u64>() {
      Ok(val) => {
        cur_sum += val;
      }
      Err(_) => {
        // We are assuming only well-structured input, so integer parse errors
        // indicate line break between groups of integers.
        if cur_sum > max_sum {
          max_sum = cur_sum;
        }
        cur_sum = 0;
      }
    }
  }
  max_sum
}

fn main() {
  let max_sum = max_group_sum(io::stdin().lines());
  println!("{}", max_sum);
}
