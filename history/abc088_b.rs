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

fn sum_a_b(mut cards: Vec<u32>) -> Vec<u32> {
  sum_a_b_logic(cards, 0, 0, true)
}

fn sum_a_b_logic(mut cards: Vec<u32>, a: u32, b: u32, is_a: bool) -> Vec<u32> {
  match cards.pop() {
    Some(n) => match is_a {
      true => sum_a_b_logic(cards, a + n, b, !is_a),
      false => sum_a_b_logic(cards, a, b + n, !is_a),
    },
    None => vec![b, a]
  }
}

fn main() {
  let n = read::<u32>();
  let mut cards = read_n::<u32>(n);
  cards.sort();
  let mut result = sum_a_b(cards);
  println!("{}", result.pop().unwrap() - result.pop().unwrap());
}
