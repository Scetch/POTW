use std::io;
use std::io::prelude::*;

fn main() {
	let input = {
		let mut buf = String::new();
		io::stdin()
			.read_to_string(&mut buf)
			.expect("Could not read from stdin.");
		buf
	};

	let mut lines = input.lines();

	let n = lines.next()
		.and_then(|v| v.parse::<usize>().ok())
		.expect("Invalid n.");

	let mut nums = lines.next()
		.map(|v| {
			v.split_whitespace()
				.take(n)
				.map(|v| {
					v.parse::<usize>()
						.expect("Invalid number in sequence.") 
				}).collect::<Vec<_>>()
		})
		.expect("Invalid number line.");

	// Find the frequency of each number and store it at the 
	// index of that number - 1.
	// We can preserve the current value of the number by
	// adding n and using mod.
	for i in 0..n {
		let v = nums[i] as usize;

		if nums[i] < n {
			nums[v - 1] += n;
		} else {
			nums[v % n - 1] += n;
		}
	}

	// Find the id with the greatest frequency.
	let mut id = 0;
	for i in 1..n {
		if nums[i] > nums[id] {
			id = i;
		}
	}

	// Process ids are from 1 to N-1.
	println!("{}", id + 1);
}
