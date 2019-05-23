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

fn all_combinations(c: &Vec<u32>, r: u32) -> Vec<u32> {
    let mut queue: Vec<u32> = vec![];
    for i in 0..c.len() {
        for j in i + 1..c.len() {
            queue.push(c[i]);
            queue.push(c[j]);
        }
    }

    let mut result: Vec<u32> = vec![];
    let max = queue.len() as u32 / r;
    for _ in 0..max {
        let mut sum = 0;
        for _ in 0..r {
            sum += queue.pop().unwrap();
        }
        result.push(sum);
    }
    result
}

fn main() {
    let mut c = read_n::<u32>(3);
    c.sort();
    let largest = c.pop().unwrap();

    match all_combinations(&c, 2).iter().any(|i| *i == largest) {
        true => println!("Yes"),
        false => println!("No"),
    };
}
