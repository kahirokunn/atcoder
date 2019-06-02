fn f(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => f(n - 2) + f(n - 1),
    }
}

fn main() {
    for i in 0..50 {
        println!("f{}, {}", i, f(i));
    }
}
