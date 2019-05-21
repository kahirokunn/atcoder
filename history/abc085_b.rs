// https://atcoder.jp/contests/abs/tasks/abc085_b

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

fn main() {
  let n = read::<u32>();
  let size_list = read_n(n);
  let active_kagami_mochi_parts: HashSet<u32> = size_list.into_iter().collect();
  println!("{}", active_kagami_mochi_parts.len());
}
