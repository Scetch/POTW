
use std::io;

/// A 2D point.
#[derive(Debug, Copy, Clone)]
struct Point(i32, i32);

impl Point {
	/// Returns the distance between two points squared.
	fn distance_sq(&self, pos: Point) -> i32 {
		(self.0 - pos.0).pow(2) + (self.1 - pos.1).pow(2)
	}
}

/// Get input from stdin and parse an i32.
/// Returns Some(i32) if the i32 is parsed successfully or None.
fn get_i32(buf: &mut String) -> Option<i32> {
	io::stdin().read_line(buf).unwrap();

	let res = {
		let mut input = buf.split_whitespace();
		input.next().and_then(|i| i.parse::<i32>().ok())
	};

	buf.clear(); // Clear the buffer.

	res
}

/// Get input from stdin and parse a point.
/// Returns Some(Point) if the point is parsed successfully or None.
fn get_point(buf: &mut String) -> Option<Point> {
	io::stdin().read_line(buf).unwrap();

	let res = {
		let mut input = buf.split_whitespace();
		let x = input.next().and_then(|x| x.parse::<i32>().ok());
		let y = input.next().and_then(|y| y.parse::<i32>().ok());

		if let (Some(x), Some(y)) = (x, y) {
			Some(Point(x, y))
		} else {
			None
		}
	};

	buf.clear(); // Clear the buffer.

	res
}

fn main() {
	let mut buf = String::new();

	// Get the users input from stdin.
	let loc = get_point(&mut buf).expect("Invalid location for william.");
	let desired = get_i32(&mut buf).expect("Invalid K.");
	let total = get_i32(&mut buf).expect("Invalid N.");

	if total < desired {
		panic!("N must be >= K");
	}
    
	let mut women = Vec::with_capacity(total as usize);

	// Read total number of points from stdin.
	for _ in 0..total {
		women.push(get_point(&mut buf).expect("Invalid location for woman."));
	}

    women.sort_by(|a, b| a.distance_sq(loc).cmp(&b.distance_sq(loc)));

    println!("-");

    // Display the first desired points to stdout.
    for &Point(x, y) in women.iter().take(desired as usize) {
    	println!("{} {}", x, y);
    }
}
