// https://atcoder.jp/contests/abs/tasks/abc081_a

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

fn count_number_1(mut words: String, count: u32) -> u32 {
  match words.pop() {
      Some(number) => match number.to_digit(core::f32::RADIX).unwrap() == 1 {
        true => count_number_1(words, count + 1),
        false => count_number_1(words, count),
      },
      None => count,
  }
}

fn main() {
  let a = read::<String>();
  println!("{}", count_number_1(a, 0));
}
