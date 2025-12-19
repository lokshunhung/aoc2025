#![allow(dead_code, unused)]

use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs::File;
use std::hash::Hash;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Lines;
use std::path::Path;
use std::slice::Windows;

struct Fixture(&'static str);
impl Fixture {
    fn reader(&self) -> BufReader<File> {
        let path = Path::new(file!()).parent().unwrap().join(self.0);
        let file = File::open(&path).unwrap();
        BufReader::new(file)
    }
}

struct Problem {
    lines: Vec<String>,
}
impl From<BufReader<File>> for Problem {
    fn from(reader: BufReader<File>) -> Self {
        let lines = reader.lines().map_while(Result::ok).collect();
        Problem { lines }
    }
}
impl Problem {
    fn solve(&mut self) -> i32 {
        let Self { lines } = self;
        let depth = lines.len();
        let mut seen = HashSet::new();

        let (mut deq, width) = {
            let first_line = lines.first().unwrap();
            let len = first_line.len();
            let beam_pos = first_line.find('S').unwrap();

            let mut deq = VecDeque::new();
            deq.push_back(vec![beam_pos]);
            (deq, len)
        };

        let mut cnt = 0;
        loop {
            let Some(mut node) = deq.pop_front() else {
                break;
            };
            if seen.contains(&node) {
                continue;
            }
            seen.insert(node.clone());

            if node.len() == depth {
                cnt += 1;
                println!("{:?}", node);
                continue;
            }

            let &pos = node.last().unwrap();
            let next_line = &lines[node.len()];

            match next_line.chars().nth(pos).unwrap() {
                '.' => {
                    let mut next_node = node;
                    next_node.push(pos);
                    deq.push_front(next_node);
                    continue;
                }
                '^' => {
                    if pos > 0 {
                        let mut left_node = node.clone();
                        left_node.push(pos - 1);
                        deq.push_front(left_node);
                    }
                    if pos < width - 1 {
                        let mut right_node = node;
                        right_node.push(pos + 1);
                        deq.push_front(right_node);
                    }
                }
                _ => panic!(),
            }
        }

        cnt
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn d7p2() {
        let fixture = Fixture("input.txt");
        let reader = fixture.reader();
        let mut problem = Problem::from(reader);
        let answer = problem.solve();

        println!("{}", answer);

        assert_eq!(answer, 3049);
    }

    #[test]
    fn sample() {
        let fixture = Fixture("sample.txt");
        let reader = fixture.reader();
        let mut problem = Problem::from(reader);
        let answer = problem.solve();

        assert_eq!(answer, 40);
    }
}
