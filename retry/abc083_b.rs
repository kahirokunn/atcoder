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

fn sum(n: u32) -> u32 {
  n.to_string().chars().map(|c| c.to_digit(10).unwrap()).sum::<u32>()
}

fn main() {
  let n = read::<u32>();
  let a = read::<u32>();
  let b = read::<u32>();

  let mut t: Vec<u32> = vec![];

  for i in 1..n+1 {
    let sum = sum(i);
    if a <= sum && sum <= b {
      t.push(i);
    }
  }

  println!("{}", t.iter().sum::<u32>());
}
