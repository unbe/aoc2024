#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! itertools = "0.13.0"
//! topo_sort = "0.4"
//! ```

use std::fs;
use std::env;
use itertools::Itertools;
use std::collections::HashMap;
use topo_sort::TopoSort;

fn main() {
  let input = fs::read_to_string(env::args().nth(1).expect("arg?")).expect("read?");
  let mut lines = input.trim().split("\n");
  let mut known = lines.by_ref().take_while(|x| *x != "").map(|x| x.split(": ").next_tuple::<_>().unwrap()).map(|(k, v)| (k, v.parse::<i32>().unwrap())).collect::<HashMap<_, _>>();
  let graph = lines.map(|x| x.split(" -> ").next_tuple::<(&str, &str)>().unwrap()).map(|t| (t.1, t.0.split(' ').collect::<Vec<_>>()))
    .collect::<HashMap<_, _>>();
  let mut topo_sort = TopoSort::<&str>::new();
  graph.iter().for_each(|(t, s)| topo_sort.insert_from_slice(t, &[s[0], s[2]]));

  let ops = HashMap::from([
      ("AND", Box::new(|a: i32, b: i32| a & b) as Box<dyn Fn(i32, i32) -> i32>),
      ("OR",  Box::new(|a: i32, b: i32| a | b) as Box<dyn Fn(i32, i32) -> i32>),
      ("XOR", Box::new(|a: i32, b: i32| a ^ b) as Box<dyn Fn(i32, i32) -> i32>),
  ]);

  let order = topo_sort.try_vec_nodes().unwrap();
  let mut part1 : u64 = 0;
  for node in order.clone() {
    let task = &graph[node];
    let r = ops[task[1]](known[task[0]], known[task[2]]);
    known.insert(node, r);
    if node.starts_with('z') && r == 1 {
      part1 += 1 << node[1..].parse::<i32>().unwrap();
    }
  }
  std::println!("part1: {}", part1);
  // Part2 solved mostly visually. See day24.svg and day24.dot
  // It was also helpful to run it with all-ones in x and y and observe
  // unexpected zeros in the result
}