// https://atcoder.jp/contests/practice/tasks/practice_1

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

fn main() {
  let a = read::<u32>();
  let b = read::<u32>();
  let c = read::<u32>();
  let s = read::<String>();
  println!("{} {}", a + b + c, s);
}
