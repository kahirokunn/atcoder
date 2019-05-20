// https://atcoder.jp/contests/abs/tasks/abc086_a

use std::io::*;
use std::str::FromStr;

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

fn is_even(a: u32, b: u32) -> bool {
  match (a * b) % 2 {
      0 => true,
      _ => false,
  }
}

fn main() {
  let a = read::<u32>();
  let b = read::<u32>();

  match is_even(a, b) {
    true => println!("Even"),
    false => println!("Odd"),
  };
}
