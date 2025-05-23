#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! itertools = "0.13.0"
//! ```

use std::fs;
use std::env;
use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;

// bron_kerbosch implementation courtesy of chatgpt (well done!)
// I claim this is at least as fair as using a library impl.
fn bron_kerbosch<'a>(
  r: &mut HashSet<&'a str>,
  p: &mut HashSet<&'a str>,
  x: &mut HashSet<&'a str>,
  graph: &HashMap<&'a str, Vec<&'a str>>,
  cliques: &mut Vec<HashSet<&'a str>>,
) {
  if p.is_empty() && x.is_empty() {
      cliques.push(r.clone());
      return;
  }

  let to_iterate: Vec<&'a str> = p.iter().copied().collect();

  for &v in &to_iterate {
      if !p.contains(v) {
          continue;
      }

      r.insert(v);

      let neighbors = graph.get(v).map(Vec::as_slice).unwrap_or(&[]);

      let mut p_new: HashSet<&'a str> =
        p.iter().copied().filter(|u| neighbors.contains(u)).collect();
      let mut x_new: HashSet<&'a str> =
        x.iter().copied().filter(|u| neighbors.contains(u)).collect();

      bron_kerbosch(r, &mut p_new, &mut x_new, graph, cliques);

      r.remove(v);
      p.remove(v);
      x.insert(v);
  }
}

fn find_maximal_cliques<'a>(graph: &'a HashMap<&str, Vec<&'a str>>) -> Vec<HashSet<&'a str>> {
  let mut r = HashSet::new();
  let mut p: HashSet<&str> = graph.keys().copied().collect();
  let mut x = HashSet::new();
  let mut cliques = Vec::new();

  bron_kerbosch(&mut r, &mut p, &mut x, graph, &mut cliques);
  cliques
}

fn main() {
  let input = fs::read_to_string(env::args().nth(1).expect("arg?")).expect("read?");
  let nums = input.trim().split("\n").map(|s| s.split("-").collect_tuple::<(&str, &str)>().unwrap()).collect::<Vec<_>>();
  let mut e = HashMap::<&str, Vec<&str>>::new();
  let mut ins = |a, b| { e.entry(a).or_insert(vec![]).push(b); };
  nums.iter().for_each(|t| { ins(t.0, t.1); ins(t.1, t.0) });
  let mut part1 = HashSet::new();
  for (n1, n2s) in e.iter() {
    if !n1.starts_with('t') {
      continue
    }
    for n2 in n2s {
      for n3 in e[n2].iter() {
        if e[n3].contains(&n1) {
          let mut nn = vec![n1, n2, n3];
          nn.sort();
          part1.insert(nn);
        }
      }
    }
  }  
  std::println!("part1: {:?}", part1.len());
  let mut mc = find_maximal_cliques(&e);
  mc.sort_by(|a, b| b.len().cmp(&a.len()));
  let mut mmc = mc[0].iter().collect::<Vec<_>>();
  mmc.sort();
  std::println!("part2: {}", mmc.iter().join(","));
}