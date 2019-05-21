// https://atcoder.jp/contests/abs/tasks/abc087_b

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
  let jp500 = read::<u32>();
  let jp100 = read::<u32>();
  let jp50 = read::<u32>();
  let sum = read::<u32>();

  let mut ans = 0;
  for i in 0..jp500+1 {
    for j in 0..jp100+1 {
      for k in 0..jp50+1 {
        if i * 500 + j * 100 + k * 50 == sum {
          ans = ans + 1;
        }
      }
    }
  }
  println!("{}", ans);
}
