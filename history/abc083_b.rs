// https://atcoder.jp/contests/abs/tasks/abc083_b

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

fn between_each_digit(n: u32, a: u32, b: u32) -> Vec<u32> {
  between_each_digit_logic(n, a, b, a, vec![])
}

fn digit_sum(n: u32) -> u32 {
  n.to_string().chars().map(|c| c.to_digit(10).unwrap()).sum::<u32>()
}

fn between_each_digit_logic(n: u32, a: u32, b: u32, c: u32, mut d: Vec<u32>) -> Vec<u32> {
  match c <= n {
    true => {
      if between(digit_sum(c), a, b) {
        d.push(c);
      }
      between_each_digit_logic(n, a, b, c + 1, d)
    },
    false => d,
  }
}

fn between(n: u32, a: u32, b: u32) -> bool {
  (a <= n) && (n <= b)
}

fn main() {
  let n = read::<u32>();
  let a = read::<u32>();
  let b = read::<u32>();
  let result = between_each_digit(n, a, b);
  println!("{}", result.iter().fold(0, |sum, i| sum + i));
}
