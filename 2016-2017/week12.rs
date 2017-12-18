
use std::io;
use std::io::prelude::*;

struct Perceptron {
	x_weight: f64,
	y_weight: f64,
	bias: f64,
	learning_rate: f64,
}

impl Perceptron {
	fn new(learning_rate: f64) -> Self {
		Perceptron {
			x_weight: 0.0,
			y_weight: 0.0,
			bias: 0.0,
			learning_rate: learning_rate,
		}
	}

	fn apply(&mut self, x: i32, y: i32) -> i32 {
		if ((self.x_weight * x as f64) + (self.y_weight * y as f64) + self.bias) > 0.0 {
			1
		} else {
			-1
		}
	}

	fn learn(&mut self, x: i32, y: i32, true_output: i32) -> i32 {
		let predicted_output = self.apply(x, y);
		let error = true_output - predicted_output;

		self.x_weight += (x * error) as f64 * self.learning_rate;
		self.y_weight += (y * error) as f64 * self.learning_rate;
		self.bias += (1 * error) as f64 * self.learning_rate;

		error
	}

	fn train_until_convergence(&mut self, data: &[(i32, i32, i32)]) {
		loop {
			if data.iter().fold(0, |i, &(x, y, o)| i + self.learn(x, y, o)) == 0 {
				break;
			}
		}
	}
}

fn main() {
    let input = {
        let mut buf = String::new();
        io::stdin().read_to_string(&mut buf).unwrap();
        buf
    };

    let mut lines = input.lines();

    let n = lines.next().and_then(|v| v.parse::<usize>().ok()).unwrap();
    let students = lines.by_ref()
    	.take(n)
    	.map(|l| {
    		let mut w = l.split_whitespace();

    		let time_spent = w.next().and_then(|v| v.parse::<i32>().ok()).unwrap();
    		let similarity = w.next().and_then(|v| v.parse::<i32>().ok()).unwrap();
    		let status = w.next().and_then(|v| v.parse::<i32>().ok()).unwrap();

    		(time_spent, similarity, status)
    	}).collect::<Vec<_>>();

    let mut p = Perceptron::new(0.1);
    p.train_until_convergence(&students);

    let m = lines.next().and_then(|v| v.parse::<usize>().ok()).unwrap();
    let first_years = lines.by_ref()
    	.take(m)
    	.map(|l| {
    		let mut w = l.split_whitespace();

    		let time_spent = w.next().and_then(|v| v.parse::<i32>().ok()).unwrap();
    		let similarity = w.next().and_then(|v| v.parse::<i32>().ok()).unwrap();

    		(time_spent, similarity)
    	});

    for (time_spent, similarity) in first_years {
    	if p.apply(time_spent, similarity) == 1 {
    		println!("Cool");
    	} else {
    		println!("Nerd");
    	}
    }
}
