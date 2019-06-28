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
  let k = read::<f64>();
  let a = read_n::<f64>(n);

  let mut max_len = 0;
  let mut l = 0;
  let mut r = 0;

  let mut sum: f64 = 1.0;
  let mut len_of_sum = 0;

  if a.iter().find(|v| (**v as u32) == 0) != None {
    println!("{}", a.len());
    return;
  }

  loop {
    if r == n {
      break;
    }

    sum *= a[r];
    len_of_sum += 1;

    if sum <= k && max_len < len_of_sum {
      max_len = len_of_sum;
    } else {
      sum /= a[l];
      l += 1;
      len_of_sum -= 1;

      if l > r {
        r = l;
      }
    }

    r += 1;
  }
  println!("{}", max_len);
}
