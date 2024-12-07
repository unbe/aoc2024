#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! itertools = "0.13.0"
//! topo_sort = "0.4"
//! ```

use std::io::{self, BufRead};
use std::env;
use std::fs::File;
use itertools::Itertools;
use std::collections::HashSet;
use topo_sort::TopoSort;

fn main() {
  let file = File::open(env::args().nth(1).expect("arg?")).expect("open");
  let mut line_iter = io::BufReader::new(file).lines().map(Result::unwrap);
  let rules = line_iter.by_ref().take_while(|s| s!="").map(|s| s.split("|").map(|d| d.parse::<i32>().unwrap()).collect_tuple().unwrap()).into_group_map();
  let updates = line_iter.map(|s| s.split(",").map(|d| d.parse::<i32>().unwrap()).collect::<Vec<_>>()).collect::<Vec<_>>(); 
  let no_deps = Vec::new();
  let mut part1_sum = 0;
  let mut part2_sum = 0;
  for up in updates {
    let rev_up = up.iter().rev().collect::<Vec<_>>();
    let topo_sort = TopoSort::from_map(rev_up.iter().map(|&&k| (k, HashSet::from_iter(rules.get(&k).unwrap_or_else(|| &no_deps).to_owned()))).collect());
    let sorted = topo_sort.try_vec_nodes().unwrap().into_iter().rev().cloned().collect::<Vec<_>>();
    if sorted == up {
      part1_sum += up[up.len() / 2];
    } else {
      part2_sum += sorted[sorted.len() / 2];
    }
  }
  println!("part1: {:?}", part1_sum);
  println!("part2: {:?}", part2_sum);
}