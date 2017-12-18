use std::collections::HashSet;
use std::io::{self, Read};

fn main() {
    let input = {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input).unwrap();
        input
    };

    let mut lines = input.lines();

    let n = lines.next()
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap();

    let dave = lines.next()
        .unwrap()
        .split(' ')
        .skip(1)
        .collect::<HashSet<_>>();
    let dave_cnt = dave.len();

    let candidate = lines.take(n - 1)
        .map(|l| {
            let mut w = l.split(' ');
            let candidate = w.next().unwrap();
            let w_cnt = w.clone().count();

            let i = w.clone()
                .filter(|w| dave.contains(w))
                .count();

            (candidate, i as f32 / (dave_cnt + w_cnt - i) as f32)
        })
        .max_by(|&(_, l), &(_, r)| l.partial_cmp(&r).unwrap())
        .map(|(candidate, _)| candidate)
        .unwrap();

    println!("{}", candidate);
}