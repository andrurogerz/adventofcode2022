use std::cmp;
use std::collections::HashMap;
use std::io;

fn update_dir_sizes(cwd : &mut Vec<String>, dirs : &mut HashMap<String, usize>, additional_size : usize) {
  for i in 0..cwd.len() {
    let dir = cwd[0..=i].join("/");
    *dirs.entry(dir).or_insert(0usize) += additional_size;
  }
}

fn parse_commands(lines : impl Iterator<Item = io::Result<String>>) -> HashMap<String, usize> {
  let mut dirs = HashMap::new();
  let mut cwd = Vec::new();
  let mut is_listing = false;
  let mut total_dir_size = 0;

  for line in lines {
    let line = line.unwrap();
    let parts : Vec<&str> = line.split_whitespace().collect();
    match parts[0] {
      "$" => { // Execute a command.
        if is_listing {
          update_dir_sizes(&mut cwd, &mut dirs, total_dir_size);
          total_dir_size = 0;
          is_listing = false;
        }
        match parts[1] {
          "cd" => {
            assert_eq!(parts.len(), 3);
            match parts[2] {
              ".." => {
                cwd.pop();
              },
              "." => {
                panic!("unexpected path token: {}", parts[2]);
              }
              _ => {
                cwd.push(parts[2].to_string());
              }
            }
          }
          "ls" => {
            assert_eq!(parts.len(), 2);
            is_listing = true;
          }
          _ => {
            panic!("unexpected command: {}", parts[1]);
          }

        }
      }
      _ => {
        assert!(is_listing);
        assert_eq!(parts.len(), 2);
        match parts[0] {
          "dir" => {
            // Ignore directories that show up on the list.
          },
          _ => {
            // Found a file with a size.
            total_dir_size += parts[0].parse::<usize>().unwrap();
          }
        }
      }
    }
  }

  // Update with any sizes from the final command.
  update_dir_sizes(&mut cwd, &mut dirs, total_dir_size);

  dirs
}

fn find_target_dir_size(dirs : &HashMap<String, usize>) -> usize {
  const TOTAL_DISK_SPACE : usize = 70000000usize;
  const REQUIRED_DISK_SPACE : usize = 30000000usize;
  let unused_disk_space = TOTAL_DISK_SPACE - *dirs.get("/").unwrap();
  assert!(unused_disk_space < REQUIRED_DISK_SPACE);

  let min_space_to_free = REQUIRED_DISK_SPACE - unused_disk_space;
  let mut candidate_dir_size = std::usize::MAX;
  for entry in dirs {
    let dir_size = *(entry.1);
    if dir_size >= min_space_to_free {
      #[cfg(debug_assertions)]
      println!("candidate dir {:?}", entry);

      candidate_dir_size = cmp::min(candidate_dir_size, dir_size);
    }
  }

  candidate_dir_size
}

fn sum_small_dirs(dirs : &HashMap<String, usize>) -> usize {
  const MAX_DIR_SIZE : usize = 100000;
  let mut dir_size_sum = 0usize;
  for entry in dirs {
    #[cfg(debug_assertions)]
    println!("directory: {:?}", entry);

    if *(entry.1) <= MAX_DIR_SIZE {
      dir_size_sum += entry.1;
    }
  }

  dir_size_sum
}

fn main() {
  let dirs = parse_commands(io::stdin().lines());

  let result = sum_small_dirs(&dirs);
  println!("part 1: {}", result);

  let result = find_target_dir_size(&dirs);
  println!("part 2: {}", result);
}
