use std::char;
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

fn convert_radix(mut n: u32, base: u32) -> u32 {
    let mut result = vec![];
    loop {
        result.push(n % base);
        n /= base;
        if base > n {
            result.push(n);
            break;
        }
    }
    result
        .iter()
        .map(|i| i.to_string())
        .rev()
        .collect::<String>()
        .parse()
        .unwrap()
}

fn main() {
    let radix = read::<u32>();
    let mut result = vec![];

    for i in 2..radix {
        let mut sum = i;
        let mut can_push = true;
        loop {
            let converted = convert_radix(sum, radix);
            if converted > 999 {
                break;
            }
            let tmp: u32 = converted
                .to_string()
                .chars()
                .map(|v| v.to_digit(10).unwrap())
                .sum();
            if tmp % i != 0 {
                can_push = false;
                break;
            }

            sum *= i;
        }
        if can_push {
            result.push(i);
        }
    }

    let max = result.len();
    for i in 0..max {
        for j in i..max {
            if result[i] > result[j] {
                let tmp = result[i];
                result[i] = result[j];
                result[j] = tmp;
            }
        }
    }
    println!(
        "{}",
        result
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
