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

fn radix(n: u32, from: u32, base: u32) -> u32 {
    let a: Vec<u32> = vec![];
    loop {
        if n == 0 {
            break
        }
        n-=1;
        a.push(1);
    }
}

fn main() {
    // sample input
    // 6
    // 1 3 2 1 1 3
    // output: 36
    let n = read::<u32>();
    let mut numbers = read_n::<u32>(n);

    // O(N^2)
    for i in 0..numbers.len() {
        for j in i + 1..numbers.len() {
            if numbers[i] < numbers[j] {
                let tmp = numbers[i];
                numbers[i] = numbers[j];
                numbers[j] = tmp;
            }
        }
    }
    let min = numbers.pop().unwrap() + 1;
    numbers.push(min);

    println!("{}", numbers.iter().fold(1, |prev, v| prev * v));
}
