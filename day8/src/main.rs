use std::cmp;
use std::io;

fn parse_grid(lines : impl Iterator<Item = io::Result<String>>) -> Vec<Vec<u8>> {
  let mut grid : Vec<Vec<u8>> = Vec::new();

  for line in lines {
    let chars : Vec<char> = line.unwrap().chars().collect();
    let mut row : Vec<u8> = Vec::with_capacity(chars.len());
    for ch in chars {
      let height = ch as u8 - '0' as u8;
      assert!(height <= 9);
      row.push(height);
    }
    grid.push(row);

    #[cfg(debug_assertions)]
    {
      // Verify same number of columns in each row.
      let len = grid.len();
      if len > 1 {
        assert_eq!(grid[len - 1].len(), grid[len - 2].len());
      }
    }

  }

  grid
}

fn trees_visible_from_perimiter(grid : &Vec<Vec<u8>>) -> usize {
  let mut visible = 0usize;

  let rows = grid.len();
  let cols = grid[0].len();

  for row in 0..rows {
    assert_eq!(cols, grid[row].len());
    for col in 0..cols {

      let mut is_visible_left = true;
      for x in 0..col {
        if grid[row][x] >= grid[row][col] {
          is_visible_left = false;
          break;
        }
      }

      let mut is_visible_right = true;
      for x in col+1..cols {
        if grid[row][x] >= grid[row][col] {
          is_visible_right = false;
          break;
        }
      }

      let mut is_visible_top = true;
      for y in 0..row {
        if grid[y][col] >= grid[row][col] {
          is_visible_top = false;
          break;
        }
      }

      let mut is_visible_bottom = true;
      for y in row+1..rows {
        if grid[y][col] >= grid[row][col] {
          is_visible_bottom = false;
        }
      }

      let is_visible = is_visible_left || is_visible_right ||
          is_visible_top || is_visible_bottom;
      if is_visible {
        visible += 1;
      }

      #[cfg(debug_assertions)]
      print!("{},", if is_visible { '1' } else { '0' });
    }

    #[cfg(debug_assertions)]
    println!("");
  }

  visible
}

fn scenic_score(grid : &Vec<Vec<u8>>) -> usize {
  let mut max_scenic_score = 0usize;

  let rows = grid.len();
  let cols = grid[0].len();

  for row in 0..rows {
    assert_eq!(cols, grid[row].len());
    for col in 0..cols {

      let mut visible_left = 0;
      for x in (0..col).rev() {
        visible_left += 1;
        if grid[row][x] >= grid[row][col] {
          break;
        }
      }

      let mut visible_right = 0;
      for x in col+1..cols {
        visible_right += 1;
        if grid[row][x] >= grid[row][col] {
          break;
        }
      }

      let mut visible_top = 0;
      for y in (0..row).rev() {
        visible_top += 1;
        if grid[y][col] >= grid[row][col] {
          break;
        }
      }

      let mut visible_bottom = 0;
      for y in row+1..rows {
        visible_bottom +=1;
        if grid[y][col] >= grid[row][col] {
          break;
        }
      }

      let scenic_score = visible_left * visible_right * visible_top * visible_bottom;

      #[cfg(debug_assertions)]
      print!("{},", scenic_score);

      max_scenic_score = cmp::max(max_scenic_score, scenic_score);
    }

    #[cfg(debug_assertions)]
    println!("");
  }

  max_scenic_score
}

fn main() {
  let grid = parse_grid(io::stdin().lines());

  let result = trees_visible_from_perimiter(&grid);
  println!("part 1: {}", result);

  let result = scenic_score(&grid);
  println!("part 2: {}", result);
}
