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

fn main() {
  let dirs = parse_commands(io::stdin().lines());
  let mut part_1 = 0usize;
  for entry in dirs {
    if entry.1 <= 100000 {
      part_1 += entry.1;
    }
  }
  println!("part 1: {}", part_1);
}
