#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! itertools = "0.13.0"
//! ```

use std::fs;
use std::env;
use std::collections::HashSet;
use itertools::Itertools;

fn main() {
  let map = fs::read_to_string(env::args().nth(1).expect("arg?")).expect("read?").trim().split("\n").map(|line| line.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
  let dirs : Vec<(i32, i32)> = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];
  let mut seen = HashSet::new();
  let sz = (map.len() as i32, map[0].len() as i32);
  let mut cost1 = 0;
  let mut cost2 = 0;
  let mut to_visit = HashSet::new();
  to_visit.insert((0,0));
  while to_visit.len() > 0 {
    let start = *to_visit.iter().next().unwrap();
    to_visit.remove(&start);
    if seen.contains(&start) {
      continue;
    }
    let typ = map[start.0 as usize][start.1 as usize];
    let mut area = 0;
    let mut walls = Vec::new();
    let mut front = HashSet::new();
    front.insert(start);
    while front.len() > 0 {
      area += 1;
      let pos = *front.iter().next().unwrap();
      front.remove(&pos);
      seen.insert(pos);
      for d in dirs.iter() {
        let next = (pos.0 as i32 + d.0, pos.1 as i32 + d.1);
        if next.0 < 0 || next.0 >= sz.0 || next.1 < 0 || next.1 >= sz.1 {
          walls.push((pos, d));
          continue;
        }
        if map[next.0 as usize][next.1 as usize] != typ {
          if !seen.contains(&next) {
            to_visit.insert(next);
          }
          walls.push((pos, d));
          continue;
        }
        if seen.contains(&next) {
          continue
        }
        front.insert((next.0, next.1));
      }
    }
    let mut sidecnt = 0;
    for plane in walls.iter().map(|(p, d)| (d, p.0*d.0.abs() + p.1*d.1.abs(), p.0*(1-d.0.abs()) + p.1*(1-d.1.abs()))).sorted().chunk_by(|(d, p, _)| (*d, *p)).into_iter().map(|(_, g)| g.map(|(_, _, v2)| v2)) {
      let mut prev = -99;
      for k in plane {
        if k - prev != 1 {
          sidecnt += 1;
        }
        prev = k;
      }
    }
    cost1 += area * walls.len();
    cost2 += area * sidecnt;
  }
  std::println!("part1: {:?}", cost1);
  std::println!("part2: {:?}", cost2);
}