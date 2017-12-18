
use std::io::prelude::*;
use std::{io, mem};

#[derive(Debug, Clone)]
struct Cell {
	v: i32,
	unique: u32,
	expr: Vec<usize>,
	dep: Vec<usize>,
}

impl Default for Cell {
	fn default() -> Self {
		Cell { v: 0, unique: 0, expr: Vec::new(), dep: Vec::new() }
	}
}

fn main() {
	let input = {
		let mut input = String::new();
		io::stdin().read_to_string(&mut input).unwrap();
		input
	};

	let mut lines = input.lines();
	let n = lines.next()
		.map(|v| v.parse::<usize>().unwrap())
		.unwrap();

	let mut cells = vec![Cell::default(); n*n];
	let mut eval = vec![]; // A stack used to keep up with what cells can be evaluated.
	
	for (idx, cell) in lines.take(n).flat_map(|l| l.split(',').take(n)).enumerate() {
		// We will first try and parse a number and if we can't it must be an expression.
		if let Ok(val) = cell.parse::<i32>() {
			cells.get_mut(idx).unwrap().v = val;
			eval.push(idx);
		} else {
			// Convert character coordinates to an array index.
			let edges = cell.split('+').map(|c| {
					let mut chs = c.chars().map(|c| c as u8 - 65);
					(chs.next().unwrap() * n as u8 + chs.next().unwrap()) as usize
				});

			for edge in edges {
				let mut u = false;

				if let Some(cell) = cells.get_mut(idx) {
					// We are checking for unique edges.
					if !cell.expr.contains(&edge) {
						cell.unique += 1;
						u = true;
					}

					cell.expr.push(edge);
				}

				match cells.get_mut(edge) {
					Some(ref mut cell) if u => cell.dep.push(idx),
					_ => (),
				}
			}
		}
	}

	// We then start evaluating cells and filling their values into other cells.
	while let Some(idx) = eval.pop() {
		// We move this vec local so we don't run into multiple bindings.
		let (dep, v) = {
			let cell = cells.get_mut(idx).unwrap();
			(mem::replace(&mut cell.dep, vec![]), cell.v)
		};

		for &dep_idx in dep.iter() {
			if let Some(cell) = cells.get_mut(dep_idx) {
				// The number of times the cell is to be summed and added.
				let n = cell.expr.iter().cloned().filter(|&i| i == idx).count();

				cell.v += n as i32 * v;
				cell.unique -= 1;

				if cell.unique == 0 {
					eval.push(dep_idx);
				}
			}
		}

		// We move the vec back.
		let _ = mem::replace(&mut cells.get_mut(idx).unwrap().dep, dep);
	}

	{
		let stdout = io::stdout();
		let mut so = stdout.lock();
		for idx in 0..n*n {
			let v = cells.get(idx).map(|c| c.v).unwrap_or(0);

			if idx % n == n - 1 {
				write!(so, "{}\n", v).unwrap();
			} else {
				write!(so, "{},", v).unwrap();
			}
		}
	}
}