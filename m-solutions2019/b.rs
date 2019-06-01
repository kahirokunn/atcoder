use std::io::*;

fn read_line() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    let s = read_line();
    let mut count = 0;

    for c in s.chars() {
        if c == 'o' {
            count += 1;
            if count >= 8 {
                break;
            }
        }
    }

    match count + 15 - s.len() >= 8 {
        true => println!("YES"),
        false => println!("NO"),
    };
}
