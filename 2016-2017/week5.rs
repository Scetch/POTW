use std::io;
use std::io::prelude::*;

#[derive(Clone)]
struct Entry {
	values: Vec<Node>,
}

#[derive(Clone)]
enum Node {
	Value(bool),
	Entry(Entry),
}

impl Node {
	fn insert<I>(&mut self, mut parts: I, val: bool)
		where I: Iterator<Item=u8>
	{
		if let Some(part) = parts.next() {
			match *self {
				Node::Value(v) => {
					let mut entry = Entry { values: vec![Node::Value(v); 256] };
					entry.values.get_mut(part as usize)
						.unwrap()
						.insert(parts, val);

					*self = Node::Entry(entry);
				}
				Node::Entry(ref mut entry) => {
					entry.values.get_mut(part as usize)
						.unwrap()
						.insert(parts, val);
				}
			}
		} else {
			*self = Node::Value(val);
		}
	}

	fn get<I>(&self, mut parts: I) -> bool 
		where I: Iterator<Item=u8>
	{
		if let Some(part) = parts.next() {
			match *self {
				Node::Value(val) => val,
				Node::Entry(ref entry) => {
					entry.values.get(part as usize)
						.unwrap()
						.get(parts)
				}
			}
		} else {
			false
		}
	}
}

struct Trie {
	root: Node,
}

impl Trie {
	fn new() -> Trie {
		Trie {
			root: Node::Value(false),
		}
	}

	fn insert(&mut self, ip: &str, val: bool) {
		let parts = ip.split(".")
			.take(4)
			.map(|v| v.parse::<u8>().unwrap());
		self.root.insert(parts, val);
	}

	fn get(&self, ip: &str) -> bool {
		let parts = ip.split(".")
			.take(4)
			.map(|v| v.parse::<u8>().unwrap());
		self.root.get(parts)
	}
}


fn main() {
	let input = {
		let mut buf = String::new();

		io::stdin()
			.read_to_string(&mut buf)
			.unwrap();

		buf
	};

	let mut lines = input.lines();

	let n = lines.by_ref()
		.next()
		.and_then(|v| v.parse::<usize>().ok())
		.unwrap();

	let mut trie = Trie::new();

	for line in lines.by_ref().take(n) {
		trie.insert(&line, true);
	}

	let m = lines.by_ref()
		.next()
		.and_then(|v| v.parse::<usize>().ok())
		.unwrap();

	for line in lines.by_ref().take(m) {
		if !trie.get(&line) {
			println!("valid");
		} else {
			println!("banned");
		}
	}
}