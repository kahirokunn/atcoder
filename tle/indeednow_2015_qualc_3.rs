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
        }
    }
}

fn flatten(nested_array: &Vec<Vec<u32>>) -> Vec<u32> {
    nested_array
        .iter()
        .flat_map(|array| array.iter())
        .cloned()
        .collect()
}

fn is_any_a_in_b(a: &Vec<u32>, b: &Vec<u32>) -> bool {
    a.iter().any(|i| b.iter().any(|j| i == j))
}

fn main() {
    let n = read::<u32>();
    let mut table: Vec<Vec<u32>> = vec![];

    for _ in 0..n - 1 {
        table.push(read_n(2));
    }
    // mut取り除く
    let table = table;

    let flatten_array: Vec<u32> = flatten(&table);

    let mut result: Vec<u32> = vec![flatten_array.iter().min().unwrap().clone()];

    loop {
        match table
            .iter()
            // 隣り合わせの頂点を絞り込む
            .filter(|row| is_any_a_in_b(&result, row))
            // 平らにする
            .flat_map(|array| array.iter())
            // 選択済みの頂点を取り除く
            .filter(|i| !result.iter().any(|j| i == &j))
            // 最小のを取得
            .min()
        {
            Some(v) => result.push(v.clone()),
            None => break,
        };
    }

    let output: String = result
        .iter()
        .map(|i| i.to_string())
        .collect::<Vec<String>>()
        .join(" ");

    println!("{}", output);
}
