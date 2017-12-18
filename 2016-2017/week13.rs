use std::io::{self, Read};

fn find_13(tree: &[i32]) -> bool {
	fn find(tree: &[i32], idx: usize, v: i32) -> bool {
		if idx < tree.len() {
			let v = tree[idx] + v;
			find(tree, 2 * idx + 1, v) || find(tree, 2 * idx + 2, v)
		} else {
			v == 13
		}
	}

	find(tree, 0, 0)
}

fn main() {
	let mut input = String::new();
	io::stdin().read_to_string(&mut input).unwrap();

	let mut lines = input.lines();

	let n = lines.next()
		.and_then(|v| v.parse::<usize>().ok())
		.unwrap();

	let tree = lines.take(n)
		.flat_map(|l| {
			l.split_whitespace()
				.map(|v| v.parse::<i32>().unwrap())
		})
		.collect::<Vec<_>>();

	if find_13(&tree) {
		println!("lucky");
	} else {
		println!("not lucky");
	}
}