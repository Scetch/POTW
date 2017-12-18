use std::io::{ self, Read, Write };
use std::collections::HashSet;

#[derive(Debug, Clone)]
struct Node<'a> {
    values: Vec<Option<Node<'a>>>,
    word: Option<&'a str>,
}

impl<'a> Node<'a> {
    fn new() -> Self {
        Self { values: vec![None; 26], word: None }
    }

    fn insert<I>(&mut self, word: &'a str, mut parts: I)
        where I: Iterator<Item = usize>
    {
        if let Some(part) = parts.next() {
            self.values.get_mut(part as usize)
                .map(|v| v.get_or_insert_with(Node::new))
                .unwrap()
                .insert(word, parts);
        } else {
            self.word = Some(word);
        }
    }
}

fn upper_idx(c: char) -> usize {
    let c = c.to_uppercase().next().unwrap();
    (c as u8 - 'A' as u8) as usize
}

#[derive(Debug)]
struct Trie<'a>(Node<'a>);

impl<'a> Trie<'a> {
	fn new() -> Self {
        Trie(Node::new())
	}

    fn insert(&mut self, word: &'a str) {
        self.0.insert(word, word.chars().map(upper_idx));
    }

    fn find(&self, word: &str) -> (bool, Option<&'a str>) {
        let mut cur = &self.0;

        for c in word.chars().map(upper_idx) {
            if let Some(v) = cur.values.get(c).and_then(|v| v.as_ref()) {
                cur = v;
            } else {
                return (false, None);
            }
        }

        (true, cur.word)
    }
}


struct WordSearch<'a> {
    rows: usize,
    cols: usize,
    mat: Vec<Vec<(char, bool)>>,
    trie: Trie<'a>,
}

impl<'a> WordSearch<'a> {
    fn with_board<I, J>(rows: usize, cols: usize, iter: I) -> WordSearch<'a>
        where I: Iterator<Item = J>,
              J: Iterator<Item = char>
    {
        let mut mat = vec![vec![('0', false); cols]; rows];

        for (y, cls) in iter.take(rows).enumerate() {
            for (x, c) in cls.take(cols).enumerate() {
                mat[y][x].0 = c;
            }
        }

        WordSearch {
            rows: rows,
            cols: cols,
            mat: mat,
            trie: Trie::new(),
        }
    }

    fn _search<F>(&mut self, buf: &mut String, i: i32, j: i32, found: &mut HashSet<&'a str>, f: &mut F)
        where F: FnMut(&'a str)
    {
        if i < 0 || j < 0 || j >= self.rows as i32 || i >= self.cols as i32 {
            return;
        }

        let (x, y) = (i as usize, j as usize);

        if self.mat[y][x].1 { return; }

        buf.push(self.mat[y][x].0);

        let (is_prefix, w) = self.trie.find(&buf);

        if is_prefix {
            match w {
                Some(w) if !found.contains(w) => {
                    found.insert(w);
                    f(w);
                }
                _ => (),
            }

            self.mat[y][x].1 = true;
            self._search(buf, i - 1, j, found, f);
            self._search(buf, i + 1, j, found, f);
            self._search(buf, i, j - 1, found, f);
            self._search(buf, i, j + 1, found, f);
            self.mat[y][x].1 = false;
        }

        buf.pop();
    }

    fn find_words<W, F>(&mut self, words: W, mut f: F)
        where W: Iterator<Item = &'a str>,
              F: FnMut(&'a str)
    {
        for word in words {
            self.trie.insert(word);
        }

        let mut found = HashSet::new();
        let mut buf = String::with_capacity(128);
        for j in 0..self.rows {
            for i in 0..self.cols {
                self._search(&mut buf, i as i32, j as i32, &mut found, &mut f);
            }
        }
    }
}

fn main() {
    let mut input = String::with_capacity(1024);
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let r = lines.next().map(str::parse).unwrap().unwrap();
    let c = lines.next().map(str::parse).unwrap().unwrap();

    let mut ws = WordSearch::with_board(
        r, c, 
        lines.by_ref()
            .map(|l| l.split_whitespace()
                .map(|w| w.split_whitespace()
                    .flat_map(str::chars)
                    .next()
                    .unwrap()
                )
            )
    );

    let n = lines.next().map(str::parse).unwrap().unwrap();

    let stdout = io::stdout();
    let mut h = stdout.lock();

    ws.find_words(lines.take(n), |w| {
        writeln!(h, "{}", w).unwrap();
    });
}