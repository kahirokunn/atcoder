// https://atcoder.jp/contests/abc054/tasks/abc054_c

use std::collections::{HashSet, VecDeque};
use std::io::*;
use std::str::FromStr;

fn read<T: FromStr>() -> T {
    let stdin = stdin();
    let stdin = stdin.lock();
    let token: String = stdin
        .bytes()
        .map(|c| c.expect("failed to read char") as char)
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !c.is_whitespace())
        .collect();
    token.parse().ok().expect("failed to parse token")
}

#[derive(Debug)]
struct Node {
    value: u32,
    children: Vec<Box<Node>>,
}

#[derive(Debug)]
struct Tree {
    root: Node,
}

type Line = Vec<u32>;

impl Tree {
    fn push(mut self, p1: u32, p2: u32) -> Self {
        let mut queue: VecDeque<Box<&mut Node>> = VecDeque::new();
        queue.push_front(Box::new(&mut self.root));

        while queue.len() > 0 {
            let node = queue.pop_front().unwrap();
            if node.value == p1 {
                node.children.push(Box::new(Node {
                    value: p2,
                    children: vec![],
                }));
            }
        }
        self
    }

    // リーフの数と通った頂点を数える
    fn full_search(self) -> (usize, Vec<Line>) {
        let mut count = 0;
        let mut lines: Vec<Line> = vec![vec![]];

        let mut queue: VecDeque<Box<Node>> = VecDeque::new();
        queue.push_back(Box::new(self.root));

        while queue.len() > 0 {
            let node = queue.pop_front().unwrap();
            lines[count].push(node.value);

            match node.children.len() > 0 {
                true => {
                    for item in node.children {
                        queue.push_back(item);
                    }
                }
                false => {
                    count += 1;
                    lines.push(vec![]);
                }
            }
        }
        (count, lines)
    }
}

fn main() {
    let s = read::<u32>();
    read::<u32>();
    let mut checker: HashSet<u32> = HashSet::new();
    // 有向木に変換できるのでそうする
    let mut tree = Tree {
        root: Node {
            value: 1,
            children: vec![],
        },
    };

    for _ in 0..s {
        let p1 = read::<u32>();
        let p2 = read::<u32>();
        checker.insert(p1);
        checker.insert(p2);
        // 木を構築
        tree = tree.push(p1, p2);
    }

    // 全ての頂点を経由している頂点だけ取得
    let (mut count, lines) = tree.full_search();
    for line in lines {
        for n in line {
            if !checker.contains(&n) {
                count -= 1;
                break;
            }
        }
    }

    println!("{}", count);
}
