use std::collections::BTreeMap;
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

struct Restaurant {
    n: u32,
    score: u32,
}

fn main() {
    let n = read::<u32>();
    let regions = {
        let mut rows = BTreeMap::<String, Vec<Restaurant>>::new();
        for i in 1..n + 1 {
            rows.entry(read::<String>())
                .or_insert(vec![])
                .push(Restaurant {
                    n: i,
                    score: read::<u32>(),
                });
        }
        for row in rows.values_mut() {
            row.sort_by(|a, b| a.score.cmp(&b.score));
            row.reverse();
        }
        rows
    };

    for region in regions.values() {
        for restaurant in region {
            println!("{}", restaurant.n);
        }
    }
}
