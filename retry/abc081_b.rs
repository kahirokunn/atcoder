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

fn count(n_list: Vec<u32>) -> u32 {
  count_logic(n_list, 0)
}

fn count_logic(n_list: Vec<u32>, count: u32) -> u32 {
  match n_list.iter().all(|&n| n != 0 && n % 2 == 0) {
    true => count_logic(n_list.iter().map(|&n| n / 2).collect(), count + 1),
    false => count
  }
}

fn main() {
  let n = read::<u32>();
  let n_list = read_n::<u32>(n);

  println!("{}", count(n_list));
}
