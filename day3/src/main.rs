use std::io;

fn item_priority(item : char) -> u8 {
  let prio : u8;
  if (item as u8 >= 'a' as u8) && (item as u8 <= 'z' as u8) {
    prio = item as u8 - 'a' as u8 + 1;
  } else if (item as u8 >= 'A' as u8) && (item as u8 <= 'Z' as u8) {
    prio = item as u8 - 'A' as u8 + 27;
  } else {
    panic!("unexpected item: {}", item);
  }
  return prio;
}

fn count_items(items : &[char], item_counts : &mut [u32; 256]) {
  for item in items {
    item_counts[*item as usize] += 1;
  }
}

fn sum_priorities(lines : impl Iterator<Item = io::Result<String>>) -> u64 {
  let mut priority : u64 = 0;
  for line in lines {
    let chars : Vec<char> = line.unwrap().chars().collect();
    let len = chars.len();

    // Line length must be even to split into two.
    assert_eq!(len % 2, 0);

    let mut item_counts_1 : [u32; 256] = [0; 256];
    let mut item_counts_2 : [u32; 256] = [0; 256];

    count_items(&chars[..len/2], &mut item_counts_1);
    count_items(&chars[len/2..], &mut item_counts_2);

    for i in 0u8..255u8 {
      let item_1_count = item_counts_1[i as usize];
      let item_2_count = item_counts_2[i as usize];
      if item_1_count > 0 && item_2_count > 0 {
        // Expect only alphabetic characters.
        assert!((i >= 'a' as u8 && i <= 'z' as u8) ||
                (i >= 'A' as u8 && i <= 'Z' as u8));
        priority += item_priority(i as char) as u64;
      }
    }
  }

  priority 
}

fn main() {
  let sum = sum_priorities(io::stdin().lines());
  println!("part 1: {}", sum);
}
