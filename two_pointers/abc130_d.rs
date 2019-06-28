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
  let k = read::<u64>();
  let a = read_n::<u64>(n);

  let mut r = 0;

  let mut sum: u64 = 0;
  let mut count = 0;

  for l in 0..n {
    while r < n && sum < k {
      // sample input
      // 5 10
      // 3 9 0 4 6

      // 1回目このループから抜ける時のカーソルの位置
      // .3 9. 0 4 6
      // countは 5 - 2 + 1 = 4追加される

      // 2回目このループに入る時、左のカーソルが一つ右に動く
      // 3 .9. 0 6 4
      // そして、このループに入る時、右のカーソルがまたkを超える値を探す
      // 3 .9 0 6. 4
      // countは 5 - 4 + 1 = 2 追加される. 合計6
      // sumはkより大きいので下のbreak文には引っかからない

      // 3回目このループから抜ける時のカーソルの位置
      // 3 9 .0 6 4.
      // countは 5 - 5 + 1 = 1追加される. 合計7

      // 4回目このループから抜ける時のカーソルの位置
      // 3 9 0 .6 4.
      // countは 5 - 5 + 1 = 1追加される. 合計8

      // 詳細解説
      // ※1
      // .3 9. 0 4 6だと、39を含む
      // 3 9 0
      // 3 9 0 4
      // 3 9 0 4 6
      // 全てが条件をクリアする

      // ※2
      // .9 0 6. 4 だと、
      // 9 0 6とこの区間を内包する9 0 6 4の2つがカウントされる

      // ※3
      // 3 9 .0 6 4. だと、
      // 0 4 6 だけがカウントされる.
      // 0 6 4より右に数値がもうないからだ

      // ※4
      // 3 9 0 .6 4. だと、
      // 6 4の区間1つだけがカウントされる
      // 6 4より右に数値がもうないからだ

      // このように左のカーソルがそこにある時に
      // みつけたその区間を内包する数値がいくつあるかを順に数えていく
      sum += a[r];
      r += 1;
    }

    if sum < k {
      break;
    }

    if sum >= k {
      count += n - r + 1;
    }

    if r == l {
      r += 1;
    } else {
      sum -= a[l];
    }
  }
  println!("{}", count);
}
