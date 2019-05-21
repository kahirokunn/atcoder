// https://atcoder.jp/contests/abs/tasks/abc085_c

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

fn count(n: i32, sum: i32) -> Option<Vec<i32>> {
  for i in 0..n+1 {
    for j in 0..n-i+1 {
      let k = n - i - j;
      if i * 10000 + j * 5000 + k * 1000 == sum {
        return Some(vec![k, j, i]);
      }
    }
  }
  None
}

fn main() {
  let n = read::<i32>();
  let sum = read::<i32>();

  match count(n, sum) {
      Some(result) => {
        let mut mut_result = result.clone();
        println!("{} {} {}", mut_result.pop().unwrap(), mut_result.pop().unwrap(), mut_result.pop().unwrap())
      },
      None => println!("-1 -1 -1")
  };
}
