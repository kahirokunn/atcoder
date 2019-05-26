use std::collections::{HashMap, HashSet};
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
    point: u32,
}

fn main() {
    let n = read::<u32>();
    let mut regions: HashMap<String, Vec<Restaurant>> = HashMap::new();
    let mut orders: Vec<String> = vec![];

    for i in 1..n + 1 {
        let key = read::<String>();
        orders.push(key.clone());
        let restaurant = regions.entry(key).or_insert(vec![]);
        restaurant.push(Restaurant {
            n: i,
            point: read::<u32>(),
        });
    }

    let orders: HashSet<String> = orders.into_iter().collect();
    let mut orders: Vec<String> = orders.into_iter().collect();
    orders.sort();

    for key in regions.values() {
        let region = regions.get_mut(key).unwrap();
        let len = region.len();
        for i in 0..len {
            for j in i..len {
                if region[i].point < region[j].point {
                    let tmp = region[j].clone();
                    *region.get_mut(j).unwrap() = region[i].clone();
                    *region.get_mut(i).unwrap() = tmp;
                }
            }
        }
    }

    for k in &orders {
        for i in 0..regions[k].len() {
            println!("{}", regions[k][i].n);
        }
    }
}
