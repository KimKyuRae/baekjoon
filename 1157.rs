use std::{io, collections::HashMap};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    let word = input.trim().to_uppercase();

    let mut cnts = HashMap::new();
    for c in word.chars() {
        *cnts.entry(c).or_insert(0) += 1;
    }

    let mut max_cnt = 0;
    let mut res = '?';

    for (chr, &cnt) in &cnts {
        if cnt > max_cnt {
            max_cnt = cnt;
            res = *chr
        } else if cnt == max_cnt {
            res = '?';
        }
    }

    println!("{}", res);
}