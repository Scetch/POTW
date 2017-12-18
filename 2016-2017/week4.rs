
use std::collections::HashMap;
use std::io;

struct WeightedQuickUnion {
	roots: Box<[usize]>,
	sizes: Box<[usize]>,
}

impl WeightedQuickUnion {
	/// Constructs a new WeightedQuickUnion.
	fn new(size: usize) -> Self {
		let mut roots = Vec::with_capacity(size);
		let mut sizes = Vec::with_capacity(size);

		for i in 0..size {
			roots.push(i);
			sizes.push(1);
		}

		WeightedQuickUnion {
			roots: roots.into_boxed_slice(),
			sizes: sizes.into_boxed_slice(),
		}
	}

	/// Determines the root of the index i.
	fn root(&self, mut i: usize) -> usize {
		while i != self.roots[i] {
			i = self.roots[i];
		}

		i
	}

	/// Determines if first has the same root as second.
	fn is_connected(&self, first: usize, second: usize) -> bool {
		self.root(first) == self.root(second)
	}

	/// Merges first and second.
	fn union(&mut self, first: usize, second: usize) {
		let i = self.root(first);
		let j = self.root(second);

		if i == j {
			return;
		}

		if self.sizes[i] < self.sizes[j] {
			self.roots[i] = j;
			self.sizes[j] += self.sizes[i];
		} else {
			self.roots[j] = i;
			self.sizes[i] += self.sizes[j];
		}
	}
}

/// Attempts to read a pair of strings from stdin.
/// Returns Some((first, second)) on success or None on failure.
fn read_pair() -> Option<(String, String)> {
	let mut buf = String::new();
	io::stdin().read_line(&mut buf).expect("Could not read from stdin.");

	let mut res = buf.split_whitespace();

	if let (Some(s1), Some(s2)) = (res.next(), res.next()) {
		Some((s1.to_owned(), s2.to_owned()))
	} else {
		None
	}
}

fn main() {
	// The number of friendships that will be entered.
	let num_friendships = {
		let mut buf = String::new();
		io::stdin().read_line(&mut buf).unwrap();
		buf.trim().parse().unwrap()
	};

	// At least double the amount of pairs.
	let mut wqu = WeightedQuickUnion::new(num_friendships * 2usize);
	let mut friend_map = HashMap::new();
	let mut cur_id = 0;

	// Read the friendship pairs.
	for _ in 0..num_friendships {
		let (s1, s2) = read_pair().expect("Invalid pair.");

		// A closure to easily add new ids or obtain ids already in the map.
		let mut or_insert_with = |name| {
			*friend_map.entry(name).or_insert_with(|| {
				let id = cur_id;
				cur_id += 1;
				id
			})
		};

		let id1 = or_insert_with(s1);
		let id2 = or_insert_with(s2);

		// Connect the two ids.
		wqu.union(id1, id2);
	}

	// The number of queries that will be entered.
	let num_queries = {
		let mut buf = String::new();
		io::stdin().read_line(&mut buf).unwrap();
		buf.trim().parse().unwrap()
	};

	let mut queries = Vec::with_capacity(num_queries);

	// Read query pairs.
	for _ in 0..num_queries {
		let (s1, s2) = read_pair().expect("Invalid pair.");

		let id1 = *friend_map.get(&s1).expect(&format!("{} is not in the group.", &s1));
		let id2 = *friend_map.get(&s2).expect(&format!("{} is not in the group.", &s2));

		queries.push((id1, id2));
	}

	// Check if the pairs are connected.
	for (id1, id2) in queries {
		if wqu.is_connected(id1, id2) {
			println!("yes");
		} else {
			println!("no");
		}
	}
}