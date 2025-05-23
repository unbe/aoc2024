#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! ```

use std::fs;
use std::env;
use std::collections::VecDeque;
use std::collections::HashMap;
use std::collections::HashSet;

fn solve(mut num : i64, rounds : i32) -> (i64, HashMap<Vec<i64>, i64>) {
  let mut last_4 = VecDeque::new();
  let mut last = 0;
  let mut sales = HashMap::new();
  for _ in 0..rounds {
    num = (num ^ num * 64) % 16777216;
    num = (num ^ num / 32) % 16777216;
    num = (num ^ num * 2048) % 16777216;  
    let price = num % 10;
    last_4.push_back(price - last);
    if last_4.len() == 4 {
      let key = last_4.iter().cloned().collect::<Vec<_>>();
      if !sales.contains_key(&key) {
        sales.insert(key, price);
      }
      last_4.pop_front();
    }
    last = price;
  }
  return (num, sales);
}

fn main() {
  let nums = fs::read_to_string(env::args().nth(1).expect("arg?")).expect("read?").trim().split("\n").map(|s| s.parse::<i64>().unwrap()).collect::<Vec<_>>();
  let solved = nums.iter().map(|&v| solve(v, 2000)).collect::<Vec<_>>();
  std::println!("part1: {}", solved.iter().map(|v| v.0).sum::<i64>());
  let mut all_keys = HashSet::new();
  solved.iter().for_each(|v| v.1.keys().for_each(|k| { all_keys.insert(k); }));
  let wins = all_keys.into_iter().map(|k| (k, solved.iter().map(|s| s.1.get(k).unwrap_or(&0)).sum::<i64>()));
  std::println!("part2: {:?}", wins.max_by(|a, b| a.1.cmp(&b.1)).unwrap().1);
}