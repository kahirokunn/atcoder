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

  let mut answer = 0;
  let mut r = 0;

  for l in 0..n {
    // 右側のカーソルを最後の要素の1つ前で止める
    // r < r+1 の条件で進められるだけカーソルを進める
    // カーソルが止まった場合、lが追いつくまで右のカーソルは待機する

    // n-1はaの最後の要素のindex番号. out of rangeしないようにn-1で止める
    while r < n - 1 && a[r] < a[r + 1] {
      r += 1;
    }

    answer += r - l + 1;

    // 右のカーソルを押し進める
    if l == r {
      r += 1;
    }
  }

  println!("{}", answer);
}
