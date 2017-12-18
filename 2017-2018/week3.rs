use std::collections::{ HashMap, HashSet };
use std::io::{ self, Read };

fn main() {
    let input = {
        let mut input = String::with_capacity(1024);
        io::stdin().read_to_string(&mut input).unwrap();
        input
    };

    let mut lines = input.lines();

    let n = lines.next()
        .and_then(|l| l.parse::<usize>().ok())
        .unwrap();

    let mut friends = HashMap::with_capacity(1024);
    for l in lines.by_ref().take(n) {
        let mut w = l.split(' ');
        let a = w.next().unwrap();
        let b = w.next().unwrap();
        friends.entry(a).or_insert(HashSet::new()).insert(b);
        friends.entry(b).or_insert(HashSet::new()).insert(a);
    }

    let m = lines.next()
        .and_then(|l| l.parse::<usize>().ok())
        .unwrap();

    let mut posts = HashMap::with_capacity(1024);
    for l in lines.by_ref().take(m) {
        let mut w = l.split(' ');
        let x = w.next().unwrap();
        let y = w.next().unwrap();
        let z = w.next().and_then(|w| w.parse::<i32>().ok()).unwrap();
        let v = posts.entry(y).or_insert(Vec::new());
        v.push((x, z));
    }
    
    let (u, t) = lines.next()
        .map(|l| {
            let mut w = l.split(' ');
            let a = w.next().unwrap();
            let b = w.next().and_then(|w| w.parse::<i32>().ok()).unwrap();
            (a, b)
        })
        .unwrap();
    
    let friends = friends.get(&u).unwrap();
    
    for (post, likes) in posts {
        let total = likes.into_iter()
            .filter(|&(user, _)| friends.contains(&user))
            .fold(0, |n, (_, like)| n + like);
            
        if total >= t {
            println!("{}", post);
        }
    }
}