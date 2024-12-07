#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! ```

use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct XY(i32, i32);

impl XY {
  fn from_tuple(xy: (i32, i32)) -> Self {
    Self(xy.0, xy.1)
  }
  fn step(&self, by: &XY) -> Self {
    Self(self.0 + by.0, self.1 + by.1)
  }
  fn rot(&self) -> Self {
    Self(self.1, -self.0)
  }
  fn is_free(&self, maze: &Vec<String>) -> bool {
    let ch = maze[self.0 as usize].as_bytes()[self.1 as usize];
    return ch == b'.' || ch == b'^'
  }
}

// returns the list of cells visited or None if it's a loop
fn simulate(maze: &Vec<String>, start_pos: &XY, start_dir: &XY, block: &XY) -> Option<HashSet<XY>> {
  let mut pos = start_pos.clone();
  let mut dir = start_dir.clone();
  let sz = XY(maze.len() as i32, maze[0].len() as i32);
  let mut visited = HashSet::new();
  loop {
    if !visited.insert((pos.clone(), dir.clone())) {
      return None
    }
    let new_pos = pos.step(&dir);
    if new_pos.0 < 0 || new_pos.0 >= sz.0 || new_pos.1 < 0 || new_pos.1 >= sz.0 { 
      break
    }
    if !new_pos.is_free(&maze) || &new_pos == block {
      dir = dir.rot();
      continue
    }
    pos = new_pos;
  }
  return Some(visited.iter().map(|p| p.0.clone()).collect::<HashSet<_>>());
}

fn main() {
    let file = File::open(env::args().nth(1).expect("arg?")).expect("open");
    let maze = io::BufReader::new(file).lines().map(Result::unwrap).collect::<Vec<_>>();
    let start_pos = XY::from_tuple(maze.iter().enumerate().flat_map(|(i, s)| -> Option<_> { Some((i as i32, s.find('^')? as i32)) }).next().unwrap());
    let start_dir = XY(-1, 0);
    let no_block = XY(-3, -3);
    let visited = simulate(&maze, &start_pos, &start_dir, &no_block).unwrap();
    println!("part1: {:?}", visited.len());

    let looping_blocks = visited.iter().filter(|block| simulate(&maze, &start_pos, &start_dir, block).is_none());
    println!("part2: {:?}", looping_blocks.count());
}
