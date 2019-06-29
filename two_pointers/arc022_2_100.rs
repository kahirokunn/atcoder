use std::cmp;
use std::collections::HashSet;
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

fn read_n<T: FromStr>(n: usize) -> Vec<T> {
  read_n_logic::<T>(n, vec![])
}

fn read_n_logic<T: FromStr>(n: usize, mut a: Vec<T>) -> Vec<T> {
  match n {
    0 => a,
    _ => {
      a.push(read::<T>());
      read_n_logic(n - 1, a)
    }
  }
}

fn main() {
  let n = read::<usize>();
  let a = read_n::<usize>(n);
  let mut max_len = 0;

  let mut r = 0;
  let mut flavor = HashSet::new();

  for l in 0..n {
    while r < n && !flavor.contains(&a[r]) {
      flavor.insert(a[r]);
      r += 1;
    }

    max_len = cmp::max(max_len, r - l);

    if l == r {
      r += 1;
    } else {
      flavor.remove(&a[l]);
    }
  }

  println!("{}", max_len);
}
