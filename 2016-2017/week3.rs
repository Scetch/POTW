
use std::collections::HashMap;
use std::io;

fn main() {
	let mut documents = Vec::new(); // Vector containing each documents terms.
	let mut buf = String::new(); // Buffer used to grab input from stdin.

	// Read the number of documents being supplied.
	let num = {
		io::stdin().read_line(&mut buf).unwrap();
		buf.trim().parse::<i32>().expect("Invalid input for the number of documents.")
	};

	// Read the documents.
	for _ in 0..num {
		buf.clear(); // Clear the buffer.
		io::stdin().read_line(&mut buf).unwrap();

		let mut map = HashMap::new();

		// Terms are seperated by whitespace.
		for term in buf.split_whitespace() {
			*map.entry(term.to_owned()).or_insert(0) += 1;
		}

		documents.push(map);
	}

	// Read the search term from stdin.
	let search = {
		buf.clear(); // Clear the buffer.
		io::stdin().read_line(&mut buf).unwrap();
		buf.trim().clone()
	};
	
	// Calculate the idf.
	let idf = {
		// Calculate the amount of documents containing search.
		let contains = documents.iter().filter(|doc| doc.contains_key(search)).count();
		(documents.len() as f64 / contains as f64).log10()
	};

	// Print the documents.
	for (index, document) in documents.iter().enumerate() {
		let count = *document.get(search).unwrap_or(&0);
		println!("{} {:.6}", index, count as f64 * idf);
	}
}
