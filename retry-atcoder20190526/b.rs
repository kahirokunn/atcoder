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

#[derive(Debug, Clone)]
struct Restaurant {
    n: u32,
    score: u32,
}

fn main() {
    let n = read::<u32>();
    let mut regions = {
        let mut rows: BTreeMap<String, Vec<Restaurant>> = BTreeMap::new();
        for i in 1..n + 1 {
            let record = rows.entry(read::<String>()).or_insert(vec![]);
            record.push(Restaurant {
                n: i,
                score: read::<u32>(),
            });
        }
        rows
    };

    for region in regions.values_mut() {
        region.sort_by(|a, b| a.score.cmp(&b.score));
        region.reverse();
    }

    for region in regions.values() {
        for restaurant in region {
            println!("{}", restaurant.n);
        }
    }
}
