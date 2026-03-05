use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let n: usize = input.trim().parse().unwrap();
    
    input.clear();

    io::stdin().read_line(&mut input).unwrap();

    let lst: Vec<i64> = input.split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect();
        
    if n == 0 {
        println!("{}", lst[0] * lst[0])
    } else {
        let min_val =  lst.iter().min().unwrap();
        let max_val = lst.iter().max().unwrap();
        println!("{}", min_val * max_val)
    }
}