
use std::io::{self, Read};

fn main() {
	let input = {
		let mut input = String::new();
		io::stdin().read_to_string(&mut input).unwrap();
		input
	};

	let mut lines = input.lines();

	let (n, m) = lines.next()
		.map(|l| {
			let mut n = l.split_whitespace()
				.take(2)
				.map(|v| v.parse::<usize>().unwrap());

			(n.next().unwrap(), n.next().unwrap())
		})
		.unwrap();
	
	let winner = lines.take(m)
		.fold(vec![(None, 0); n], |mut districts, l| {
			let mut parts = l.split_whitespace().take(2);

			let name = parts.next().unwrap();
			let id = parts.next()
				.map(|v| v.parse::<usize>().unwrap())
				.unwrap();

			if let Some(district) = districts.get_mut(id) {
				match district.0 {
					Some(n) if name == n => district.1 += 1,
					Some(_) => {
						district.1 -= 1;

						if district.1 == 0 {
							*district = (Some(name), 1);
						}
					}
					_ => *district = (Some(name),  1),
				}
			}

			districts
		})
		.iter()
		.fold((None, 0), |(mut candidate, mut counter), &(name, _)| {
			match name {
				n if counter == 0 => {
					candidate = n;
					counter += 1;
				}
				n if candidate == n => counter += 1,
				_ => counter -= 1,
			}

			(candidate, counter)
		});

	if let (Some(name), _) = winner {
		println!("{}", name);
	}
}