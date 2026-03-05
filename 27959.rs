use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let v: Vec<i32> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();

    println!("{}", if v[0] * 100 >=  v[1] {"Yes"} else {"No"})
}