#![allow(dead_code, unused)]

use std::cell::RefCell;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Lines;
use std::path::Path;
use std::rc::Rc;

#[derive(Debug)]
struct Fixture(&'static str);
impl Fixture {
    fn reader(&self) -> BufReader<File> {
        let path = Path::new(file!()).parent().unwrap().join(self.0);
        let file = File::open(&path).unwrap();
        BufReader::new(file)
    }
}

#[derive(Clone, Debug)]
struct Edge {
    distance: f64,
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Input {
    junctions: Vec<(i64, i64, i64)>,
}
impl TryFrom<&mut Lines<BufReader<File>>> for Input {
    type Error = String;

    fn try_from(lines: &mut Lines<BufReader<File>>) -> Result<Self, Self::Error> {
        let mut junctions = Vec::new();

        for line in lines.map_while(Result::ok) {
            let fail = || format!("cannot parse line {}", line);
            let mut s = line.split(',').map(|s| s.parse::<i64>().ok()).flatten();
            let x = s.next().ok_or_else(fail)?;
            let y = s.next().ok_or_else(fail)?;
            let z = s.next().ok_or_else(fail)?;
            junctions.push((x, y, z));
        }

        Ok(Input { junctions })
    }
}
impl Input {
    fn solve(self, mut num_connect: usize) -> i64 {
        let Self { junctions } = self;

        let edges = {
            let mut edges = Vec::new();
            for (x, j1) in junctions.iter().enumerate() {
                for (y, j2) in junctions.iter().enumerate().skip(x + 1) {
                    let mut distance = 0_f64;
                    distance += (j1.0 - j2.0).pow(2) as f64;
                    distance += (j1.1 - j2.1).pow(2) as f64;
                    distance += (j1.2 - j2.2).pow(2) as f64;
                    distance = distance.sqrt();
                    edges.push(Edge { distance, x, y });
                }
            }
            edges.sort_by(|a, b| a.distance.total_cmp(&b.distance));
            edges
        };
        for edge in &edges {
            println!(
                "{}, {} -> {} :: {:?}, {:?}",
                edge.x, edge.y, edge.distance, junctions[edge.x], junctions[edge.y]
            );
        }

        let circuits = {
            let mut circuits = Vec::new();
            let mut connections = HashMap::<usize, Rc<RefCell<BTreeSet<usize>>>>::new();
            let mut iter = edges.iter();
            loop {
                if num_connect == 0 {
                    break;
                }
                let Some(&Edge { x, y, .. }) = iter.next() else {
                    break;
                };

                let xs = connections.get(&x);
                let ys = connections.get(&y);

                match (xs, ys) {
                    (Some(_), Some(_)) => todo!(),
                    (None, Some(nodes)) => {
                        num_connect -= 1;
                        nodes.borrow_mut().insert(x);
                        connections.insert(x, nodes.clone());
                    }
                    (Some(nodes), None) => {
                        num_connect -= 1;
                        nodes.borrow_mut().insert(y);
                        connections.insert(y, nodes.clone());
                    }
                    (None, None) => {
                        let nodes = Rc::new(RefCell::new(BTreeSet::from([x, y])));
                        connections.insert(x, nodes.clone());
                        connections.insert(y, nodes.clone());
                        circuits.push(nodes);
                    }
                }
            }
            circuits
        };

        0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample() {
        let fixture = Fixture("sample.txt");
        let mut lines = fixture.reader().lines();
        let input = Input::try_from(&mut lines).unwrap();
        let answer = input.solve(9);

        assert_eq!(answer, 40);
    }
}
