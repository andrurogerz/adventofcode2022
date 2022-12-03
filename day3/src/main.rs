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

fn sum_priorities(lines : impl Iterator<Item = io::Result<String>>) -> (u64, u64) {
  let mut priority : u64 = 0;
  let mut badge_priority : u64 = 0;
  let mut group_index : usize = 0;
  let mut group_counts : [[u32; 256]; 3] = [[0; 256]; 3];

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

    count_items(&chars, &mut group_counts[group_index]);

    group_index += 1;
    if group_index == 3 {
      for i in 0u8..255u8 {
        if group_counts[0][i as usize] > 0 &&
           group_counts[1][i as usize] > 0 &&
           group_counts[2][i as usize] > 0 {
          badge_priority += item_priority(i as char) as u64;
        }
      }
      group_counts = [[0 ; 256] ; 3];
      group_index = 0;
    }
  }

  // Expect no leftover lines, must be 3 per group.
  assert_eq!(group_index, 0);

  (priority, badge_priority)
}

fn main() {
  let (priority, badge_priority) = sum_priorities(io::stdin().lines());
  println!("part 1: {}", priority);
  println!("part 2: {}", badge_priority);
}
