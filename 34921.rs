use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let lst: Vec<i64> = input
        .split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect();
    let a = lst[0];
    let t = lst[1];
    let result = 10 + 2 * (25 - a + t);
    if result < 0 {
        println!("0");
    } else {
        println!("{}", result);
    }
}