use std::collections::{ HashSet, HashMap };
use std::io::{ self, Read };

fn cyclic(g: &HashMap<usize, HashSet<usize>>, start: usize) -> bool {
    let mut stack = vec![(start, HashSet::new())];
    
    while let Some((v, mut seen)) = stack.pop() {
        if seen.contains(&v) {
            return true;
        }
        
        seen.insert(v);
        
        stack.extend(
            g.get(&v)
                .iter()
                .flat_map(|l| l.iter().map(|n| (*n, seen.clone())))
        );
    }
    
    false
}

fn main() {
    let mut input = String::with_capacity(1024);
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    
    let _ = lines.next();
    let n = lines.next().map(str::parse).unwrap().unwrap();
    
    let mut graph = HashMap::new();
    
    for mut l in lines.by_ref().take(n).map(str::split_whitespace) {
        let a = l.next().map(str::parse).unwrap().unwrap();
        let b = l.next().map(str::parse).unwrap().unwrap();
        graph.entry(a).or_insert(HashSet::new()).insert(b);
    }
        
    let cyclic = graph.keys().any(|k| cyclic(&graph, *k));
    println!("{}", if cyclic { "Bad" } else { "Good" });
}