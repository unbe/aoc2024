#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! memoize = "0.4.2"
//! ```

use std::fs;
use std::env;
use memoize::memoize;

#[memoize]
fn solve(i : u64, rounds : i32) -> usize {
  if rounds == 0 {
    return 1
  }
  if i == 0 {
    return solve(1, rounds - 1)
  }
  let digs = i.ilog10() + 1;
  if digs % 2 == 0 {
    let div = 10_u64.pow(digs/2);
    return solvec(&vec![i / div, i % div], rounds - 1)
  }
  return solve(i * 2024, rounds - 1);
}

fn solvec(nums : &Vec<u64>, rounds : i32) -> usize {
  if rounds == 0 {
    return nums.len();
  }
  return nums.iter().map(|&v| solve(v, rounds)).sum();
}

fn main() {
  let nums = fs::read_to_string(env::args().nth(1).expect("arg?")).expect("read?").trim().split(" ").map(|s| s.parse::<u64>().unwrap()).collect::<Vec<_>>();
  std::println!("part1: {}", solvec(&nums, 25));
  std::println!("part2: {}", solvec(&nums, 75));
}