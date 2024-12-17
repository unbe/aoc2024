#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! regex = "1.11.1"
//! ndarray = "0.15"
//! ndarray-linalg = { version = "0.15", features = ["openblas-system"] }
//! ```

use std::fs;
use std::env;
use regex::Regex;
use ndarray::*;
use ndarray_linalg::*;

fn main() {
  let fin = fs::read_to_string(env::args().nth(1).expect("arg?")).expect("read?");
  let button_re = Regex::new(r"^Button .: X.(\d+), Y.(\d+)$").unwrap();
  let prize_re = Regex::new(r"^Prize: X.(\d+), Y.(\d+)$").unwrap();
  let mut moves = vec![];
  let mut total1 = 0;
  let mut total2 = 0;
  for line in fin.trim().split("\n") {
    let button = button_re.captures_iter(line).next().map(|c| c.extract::<2>().1.map(|s| s.parse::<f64>().unwrap()));
    match button {
      Some(mv) => moves.push(mv),
      None => (),
    }
    let prize = prize_re.captures_iter(line).next().map(|c| c.extract::<2>().1.map(|s| s.parse::<f64>().unwrap()));
    match prize {
      Some(pr) => { 
        let solve_prize = |pr : [f64; 2]| {
          let x = arr2(&moves).t().solve(&arr1(&pr)).unwrap();
          if x.map(|r| (r-r.round()).abs() < (r/1e14)).iter().all(|z| *z) {
            let res = x.map(|r| r.round() as i64).to_vec();
            return res[0]*3 + res[1];
          }
          return 0;
        };
        total1 += solve_prize(pr);
        total2 += solve_prize(pr.map(|x| x + 10000000000000.0));
        moves = vec![];
      }
      None => (),
    }
  }
  println!("part1: {}", total1);
  println!("part2: {}", total2);
}