#![allow(dead_code, unused)]

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Lines;
use std::path::Path;

#[derive(Debug)]
struct Fixture(&'static str);
impl Fixture {
    fn reader(&self) -> BufReader<File> {
        let path = Path::new(file!()).parent().unwrap().join(self.0);
        let file = File::open(&path).unwrap();
        BufReader::new(file)
    }
}

#[derive(Debug)]
struct Problem {
    lines: Vec<String>,
    ops_line: OpsLine,
}
impl TryFrom<&Fixture> for Problem {
    type Error = String;

    fn try_from(fixture: &Fixture) -> Result<Self, Self::Error> {
        let mut lines = Vec::new();
        let mut ops_line = None;

        for line in fixture.reader().lines().map_while(Result::ok) {
            if line.is_empty() {
                continue;
            }
            match Line::try_from(line)? {
                Line::Raw(string) => {
                    lines.push(string);
                }
                Line::Ops(line) => {
                    ops_line = Some(line);
                    break;
                }
            };
        }

        if lines.is_empty() {
            return Err(format!("no lines found in {}", fixture.0));
        };
        let Some(ops_line) = ops_line else {
            return Err("missing ops line".to_owned());
        };

        Ok(Self { lines, ops_line })
    }
}
impl Problem {
    fn solve(self) -> u64 {
        let Self { lines, ops_line } = self;

        let mut pad_start = 0;
        let mut result: u64 = 0;

        for (op, pad) in ops_line.iter() {
            let mut nums: Vec<u64> = vec![0; pad + 1];

            for line in &lines {
                let c = &line[pad_start..=pad_start + pad];
                for (i, char) in c.chars().enumerate() {
                    let Some(digit) = char.to_digit(10) else {
                        continue;
                    };
                    nums[i] = nums[i] * 10 + digit as u64;
                }
            }
            println!("{:?}", nums);

            pad_start += pad + 2; // 1 for op_sign, 1 for padding

            result += match op {
                Op::Add => nums.iter().sum::<u64>(),
                Op::Multiply => nums.iter().product::<u64>(),
            };
        }

        result
    }
}

#[derive(Debug)]
enum Line {
    Raw(String),
    Ops(OpsLine),
}
impl TryFrom<String> for Line {
    type Error = String;
    fn try_from(string: String) -> Result<Self, Self::Error> {
        match OpsLine::try_from(string.as_str()) {
            Ok(ops_line) => return Ok(Self::Ops(ops_line)),
            Err(OpsLineErr::ParseErr(line)) => return Err(format!("error parsing ops line {}", line)),
            Err(OpsLineErr::NotOpLine) => {}
        };
        Ok(Line::Raw(string))
    }
}

#[derive(Debug)]
enum OpsLineErr {
    NotOpLine,
    ParseErr(String),
}

#[derive(Debug, PartialEq)]
struct OpsLine(Vec<(Op, usize)>);
impl TryFrom<&str> for OpsLine {
    type Error = OpsLineErr;

    fn try_from(line: &str) -> Result<Self, Self::Error> {
        let first_char = line.chars().nth(0).ok_or(OpsLineErr::NotOpLine)?;
        if Op::try_from(first_char).is_err() {
            return Err(OpsLineErr::NotOpLine);
        }

        let mut ops: Vec<(Op, usize)> = Vec::new();

        for char in line.chars() {
            if char == ' ' {
                let last = ops.last_mut().ok_or(OpsLineErr::ParseErr(line.to_owned()))?;
                last.1 += 1;
                continue;
            }

            if let Ok(op) = Op::try_from(char) {
                ops.push((op, 0));
                continue;
            }

            return Err(OpsLineErr::ParseErr(line.to_owned()));
        }

        for op in &mut ops.iter_mut().rev().skip(1) {
            op.1 -= 1;
        }

        Ok(Self(ops))
    }
}
impl OpsLine {
    fn iter(&self) -> impl Iterator<Item = &(Op, usize)> {
        self.0.iter()
    }
}

#[derive(Debug, PartialEq)]
enum Op {
    Add,
    Multiply,
}
impl TryFrom<char> for Op {
    type Error = ();
    fn try_from(value: char) -> Result<Self, Self::Error> {
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
    fn d6p2() {
        let fixture = Fixture("input.txt");
        let problem = Problem::try_from(&fixture).unwrap();
        let answer = problem.solve();

        println!("{}", answer);

        assert_eq!(answer, 11386445308378);
    }

    #[test]
    fn sample() {
        let fixture = Fixture("sample.txt");
        let problem = Problem::try_from(&fixture).unwrap();
        println!("{:?}", problem);
        let answer = problem.solve();

        assert_eq!(answer, 3263827);
    }

    #[test]
    fn parse_op_line() {
        let ops_line = OpsLine::try_from("*   +   *   +  ");
        assert!(ops_line.is_ok());

        let ops_line = ops_line.unwrap();
        assert_eq!(
            ops_line,
            OpsLine(vec![(Op::Multiply, 2), (Op::Add, 2), (Op::Multiply, 2), (Op::Add, 2),])
        );
    }
}
