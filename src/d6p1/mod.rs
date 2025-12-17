#![allow(dead_code, unused)]

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Lines;
use std::path::Path;

struct Fixture(&'static str);
impl Fixture {
    fn reader(&self) -> BufReader<File> {
        let path = Path::new(file!()).parent().unwrap().join(self.0);
        let file = File::open(&path).unwrap();
        BufReader::new(file)
    }
}

struct Problem<'a> {
    lines: &'a mut Lines<BufReader<File>>,
}
impl<'a> From<&'a mut Lines<BufReader<File>>> for Problem<'a> {
    fn from(lines: &'a mut Lines<BufReader<File>>) -> Self {
        Problem { lines }
    }
}
impl<'a> Problem<'a> {
    fn solve(&mut self) -> u64 {
        let mut columns = Vec::new();

        let ops = loop {
            let line = self.lines.next().unwrap().unwrap();
            let line = Line::from(line.as_str());
            match line {
                Line::Nums(nums) => {
                    if columns.is_empty() {
                        columns = vec![vec![]; nums.len()];
                    }
                    for (i, num) in nums.iter().enumerate() {
                        columns[i].push(*num);
                    }
                    continue;
                }
                Line::Ops(ops) => break ops,
            }
        };

        columns
            .iter()
            .enumerate()
            .map(|(i, nums)| match ops[i] {
                Op::Add => nums.iter().sum::<u64>(),
                Op::Multiply => nums.iter().product::<u64>(),
            })
            .sum::<u64>()
    }
}

enum Line {
    Nums(Vec<u64>),
    Ops(Vec<Op>),
}
impl From<&str> for Line {
    fn from(value: &str) -> Self {
        if Op::try_from(value).is_ok() {
            let ops = value
                .split(" ")
                .filter_map(|s| Op::try_from(s).ok())
                .collect::<Vec<_>>();
            return Line::Ops(ops);
        }

        let nums = value
            .split(" ")
            .filter_map(|s| s.trim().parse::<u64>().ok())
            .collect::<Vec<_>>();
        Line::Nums(nums)
    }
}

enum Op {
    Add,
    Multiply,
}
impl TryFrom<&str> for Op {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.trim().chars().nth(0).ok_or(())?;
        match value {
            '+' => Ok(Op::Add),
            '*' => Ok(Op::Multiply),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn d6p1() {
        let fixture = Fixture("input.txt");
        let mut lines = fixture.reader().lines();
        let mut problem = Problem::from(&mut lines);
        let answer = problem.solve();

        println!("{}", answer);

        assert_eq!(answer, 5873191732773);
    }

    #[test]
    fn sample() {
        let fixture = Fixture("sample.txt");
        let mut lines = fixture.reader().lines();
        let mut problem = Problem::from(&mut lines);
        let answer = problem.solve();

        assert_eq!(answer, 4277556);
    }
}
