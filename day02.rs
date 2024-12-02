use std::io::{self, BufRead};
use std::env;
use std::fs::File;

fn is_safe(vv : Vec<i32>) -> bool {
  let mut v = vv;
  if !v.is_sorted() {
    v.reverse();
  }
  if !v.is_sorted() {
    return false;
  }
  return v.windows(2).all(|z| { let d = z[1]-z[0]; d >= 1 && d <= 3});
}

fn main() {
  let file = File::open(env::args().nth(1).expect("arg?")).expect("open");
  let mut safe = 0;
  let mut cond_safe = 0;
  for line in io::BufReader::new(file).lines() {
    let v = line.unwrap().split(" ").map(|a| a.parse::<i32>().unwrap()).collect::<Vec<_>>();
    if is_safe(v.clone())  {
      safe += 1;
    } else if (0..v.len()).map(
        |drop| v.clone().into_iter().enumerate().filter_map(|(i, el)| (i != drop).then(|| el)).collect::<Vec<_>>()
      ).any(is_safe) {
      cond_safe += 1;
    }
  }
  println!("part1: {}", safe);
  println!("part2: {}", safe + cond_safe);
}