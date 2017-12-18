
use std::collections::HashMap;
use std::hash::Hash;
use std::io;
use std::io::prelude::*;

struct DisjointSet<T>(HashMap<T, (T, usize)>);

impl<T> DisjointSet<T>
    where T: Eq + Hash + Copy
{
    fn new() -> Self {
        DisjointSet(HashMap::new())
    }

    fn union(&mut self, u: T, v: T) {
        if u != v {
            let (p1, p2) = (self.find(u), self.find(v));
            let (r1, r2) = (self.0.get(&p1).unwrap().1, self.0.get(&p2).unwrap().1);

            if r1 >= r2 {
                self.0.get_mut(&p2).unwrap().0 = p1;
                self.0.get_mut(&p1).unwrap().1 += r2;
            } else {
                self.0.get_mut(&p1).unwrap().0 = p2;
                self.0.get_mut(&p2).unwrap().1 += r1;
            }
        }
    }

    fn find(&mut self, u: T) -> T {
        let v = self.0.entry(u).or_insert((u, 1)).0;

        if v == u {
            u
        } else {
            let v = self.find(v);
            self.0.get_mut(&u).unwrap().0 = v;
            v
        }
    }

    fn is_connected(&mut self, u: T, v: T) -> bool {
        (self.0.contains_key(&u) && self.0.contains_key(&v)) && (self.find(u) == self.find(v))
    }
}

fn main() {
    // Get input from stdin
    let input = {
        let mut buf = String::new();
        io::stdin()
            .read_to_string(&mut buf)
            .expect("Couldn't read from stdin.");
        buf
    };

    let mut lines = input.lines();
    
    // Number of rows and cols
    let n = lines.next()
        .and_then(|v| v.parse::<usize>().ok())
        .expect("Invalid N.");

    // Converts n lines of n cols to n*n of (x, y, cost) edges
    let mut edges = lines.take(n)
        .enumerate()
        .flat_map(|(j, l)| {
            l.split_whitespace()
                .take(n)
                .map(|c| c.parse::<usize>().expect("Invalid cost."))
                .enumerate()
                .filter_map(move |(i, c)| {
                    if c == 0 { None } else { Some((i, j, c)) }
                })
        })
        .collect::<Vec<_>>();

    // Sort edges by cost
    edges.sort_by(|a, b| a.2.cmp(&b.2));

    let mut set = DisjointSet::new();
    let mut total_cost = 0;

    for (i, j, cost) in edges {
        if !set.is_connected(i, j) {
            total_cost += cost;
            set.union(i, j);
        }
    }
    
    println!("{:?}", total_cost);
}
