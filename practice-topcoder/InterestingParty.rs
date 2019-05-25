use std::collections::HashMap;
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

// O(N)
fn main() {
    // sample input
    // 4
    // snakes programing cobra monty
    // python python anaconda python

    let n = read::<usize>();
    let s = read_n::<String>((n * 2) as u32);
    let mut dict: HashMap<String, u32> = HashMap::new();

    for w in s {
        let v = dict.entry(w).or_insert(1);
        *v += 1;
    }

    println!("{:?}", dict.values().max().unwrap());
}
