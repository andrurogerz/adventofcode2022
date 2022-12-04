use std::io;

fn parse_ranges(lines : impl Iterator<Item = io::Result<String>>) -> Vec<[(usize, usize); 2]>{
  let mut range_pairs : Vec<[(usize, usize); 2]> = Vec::new();
  range_pairs.push([(1, 2), (3, 4)]);

  for line in lines {
    let line = line.unwrap();
    let ranges : Vec<&str> = line.split(',').collect();
    assert_eq!(ranges.len(), 2); // Always only two entries.

    let mut entry : [(usize, usize); 2] = [(0, 0); 2];
    for i in 0..2 {
      let parts : Vec<&str> = ranges[i].split('-').collect();
      assert_eq!(parts.len(), 2); // Always only two entries.

      entry[i] = (parts[0].parse::<usize>().unwrap(),
                  parts[1].parse::<usize>().unwrap());
      assert!(entry[i].0 <= entry[i].1); // Range must be valid.

    }
    range_pairs.push(entry);
  }

  range_pairs
}

fn count_fully_contained_range_pairs(range_pairs : &Vec<[(usize, usize); 2]>) -> usize {
  let mut count = 0usize;
  for entry in range_pairs {
    if (entry[0].0 >= entry[1].0 && entry[0].1 <= entry[1].1) ||
       (entry[1].0 >= entry[0].0 && entry[1].1 <= entry[0].1) {
      count += 1;
    }
  }

  count
}

fn count_overlapping_range_pairs(range_pairs : &Vec<[(usize, usize); 2]>) -> usize {
  let mut count = 0usize;
  for entry in range_pairs {
    if entry[0].0 <= entry[1].1 && entry[1].0 <= entry[0].1 {
      count += 1;
    }
  }

  count
}

fn main() {
  let range_pairs = parse_ranges(io::stdin().lines());
  let count = count_fully_contained_range_pairs(&range_pairs);
  println!("part 1: {}", count);

  let count = count_overlapping_range_pairs(&range_pairs);
  println!("part 2: {}", count);
}
