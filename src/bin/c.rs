#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
use std::collections::HashMap;
#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut map = HashMap::new();
    let mut ans = 0;
    for _ in 0..n {
        input! {
            mut s: Chars,
        }
        s.sort();
        let sorted_string = s.into_iter().rev().collect::<String>();
        map.entry(sorted_string)
            .and_modify(|e| *e += 1)
            .or_insert(1);
    }

    for (_, v) in &map {
        ans += v * (v - 1) / 2;
    }

    println!("{}", ans);
}
