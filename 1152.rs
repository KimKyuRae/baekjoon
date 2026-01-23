use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let len: i32 = input.split_whitespace().count().try_into().unwrap();

    println!("{}", len);
}