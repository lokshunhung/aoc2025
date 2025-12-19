#![allow(dead_code, unused)]

use std::cmp::max;
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
    fn solve(&mut self) -> u64 {
        let Self { lines } = self;
        let y_size = lines.len();
        let x_size = lines.first().unwrap().len();

        let mut dp = vec![vec![0u64; x_size]; y_size];

        for (y, line) in lines.iter().enumerate() {
            for (x, char) in line.chars().enumerate() {
                match char {
                    'S' => {
                        dp[y][x] = 1;
                    }
                    '.' => {
                        if y > 0 {
                            dp[y][x] += dp[y - 1][x];
                        }
                    }
                    '^' => {
                        let cols = [
                            if x > 0 { Some(x - 1) } else { None },
                            if x < x_size - 1 { Some(x + 1) } else { None },
                        ];
                        for &col in cols.iter().flatten() {
                            let v1 = dp[y][col];
                            let v2 = dp[y - 1][x] + dp[y][col];
                            dp[y][col] = max(v1, v2);
                        }
                    }
                    _ => panic!(),
                }

                // for line in &dp {
                //     println!("{:?}", line);
                // }
                // println!("({}, {})", x, y);
                // println!();
            }
        }

        dp.last().unwrap().iter().sum()
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

        assert_eq!(answer, 231507396180012);
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
