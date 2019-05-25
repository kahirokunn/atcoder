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

fn min<T: PartialOrd>(a: T, b: T) -> T {
    match a > b {
        true => b,
        false => a,
    }
}

fn main() {
    let bn = read::<u32>();
    let capacities = read_n::<u32>(bn);
    let mut bottles = read_n::<u32>(bn);

    let mn = read::<u32>();
    let from_ids = read_n::<u32>(mn);
    let to_ids = read_n::<u32>(mn);

    for i in 0..mn {
        let f = from_ids[i as usize] as usize;
        let t = to_ids[i as usize] as usize;
        let space = capacities[t] - bottles[t];
        let vol = min(space, bottles[f]);
        bottles[t] += vol;
        bottles[f] -= vol;
    }

    println!("{:?}", bottles);
}
