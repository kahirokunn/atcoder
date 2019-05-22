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
  let m500 = read::<u32>();
  let m100 = read::<u32>();
  let m50 = read::<u32>();
  let x = read::<u32>();

  let mut count: u32 = 0;

  for n500 in 0..m500+1 {
    for n100 in 0..m100+1 {
      for n50 in 0..m50+1 {
        if n500 * 500 + n100 * 100 + n50 * 50 == x {
          count+=1;
        }
      }
    }
  }

  println!("{}", count);
}
