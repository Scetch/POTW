
use std::io::prelude::*;
use std::io;

const INF: i32 = 9999;

fn dijsktra(graph: &[i32], n: usize, mut start: usize, end: usize) -> i32 {
	let mut node = vec![(INF, false); n];
	node[start] = (0, true);

	while !node[end].1 {
		let mut min = INF;
		let mut m = 0;

		for i in 0..n {
			let d = node[start].0 + graph[start * n + i];

			if !node[i].1 {
				if d < node[i].0 {
					node[i].0 = d;
				}

				if min > node[i].0 {
					min = node[i].0;
					m = i;
				}
			}
		}

		start = m;
		node[start].1 = true;
	}

	node[end].0
}

fn main() {
	let input = {
		let mut input = String::new();
		io::stdin().read_to_string(&mut input).unwrap();
		input
	};

	let mut lines = input.lines();

	let start_word = lines.next()
		.unwrap();

	let n = lines.next()
		.map(|v| v.parse::<usize>().unwrap())
		.unwrap();

	let mut start = 0;
	let mut end = 0;

	let mut words = Vec::with_capacity(n);
	for (i, w) in lines.take(n).enumerate() {
		if w == start_word { start = i };
		if w == "GEEK" { end = i };

		words.push(w);
	}

	let mut graph = vec![0; n * n];
	for i in 0..n {
		for j in 0..n {
			if i == j { continue; }

			let diff = words[i].chars()
				.zip(words[j].chars())
				.fold(0, |diff, (c1, c2)| {
					if c1 != c2 { diff + 1 } else {	diff }
				});

			graph[i * n + j] = if diff == 1 { 1 } else { INF };
		}
	}

	println!("{}", dijsktra(&graph, n, start, end))
}