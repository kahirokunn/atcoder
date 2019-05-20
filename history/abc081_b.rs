// https://atcoder.jp/contests/abs/tasks/abc081_b

use std::io::*;
use std::str::FromStr;
extern crate core;

fn read<T: FromStr>() -> T {
    let stdin = stdin();
    let stdin = stdin.lock();
    let token: String = stdin
        .bytes()
        .map(|c| c.expect("failed to read char") as char)
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !c.is_whitespace())
        .collect();
    token.parse().ok().expect("failed to parse token")
}

fn read_n<T: FromStr>(n: u32) -> Vec<T> {
    read_n_logic::<T>(n, vec![])
}

fn read_n_logic<T: FromStr>(n: u32, mut a: Vec<T>) -> Vec<T> {
    match n {
        0 => a,
        _ => {
            a.push(read::<T>());
            read_n_logic(n - 1, a)
        },
    }
}

fn divided_even_n(n: u32) -> u32 {
  divided_even_n_logic(n, 0)
}

fn divided_even_n_logic(n: u32, divided_n: u32) -> u32 {
  match is_even(n) {
      true => divided_even_n_logic(n / 2, divided_n + 1),
      false => divided_n,
  }
}

fn is_even(a: u32) -> bool {
  a % 2 == 0
}

fn main() {
  let a_num = read::<u32>();
  let a = read_n::<u32>(a_num);
  let mut history: Vec<u32> = vec![];

  for n in &a {
    history.push(divided_even_n(n.clone()));
  }
  println!("{}", history.iter().min().unwrap());
}
