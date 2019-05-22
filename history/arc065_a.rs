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

fn main() {
    let patterns = ["eraser", "erase", "dreamer", "dream"];
    let s = patterns.iter().fold(read::<String>(), |s, x| s.replace(x, ""));
    println!("{}", if s.is_empty() { "YES" } else { "NO "});
}
