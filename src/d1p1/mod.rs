#![allow(dead_code)]

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

#[derive(Debug)]
struct Dial {
    current: i32,
}

impl Dial {
    fn new() -> Self {
        Self { current: 50 }
    }

    fn accept(&mut self, line: &Line) {
        let next = self.current + line.delta();
        self.current = next.rem_euclid(100);
    }
}

#[derive(Debug)]
enum Dir {
    L,
    R,
}

#[derive(Debug)]
struct Line {
    dir: Dir,
    clicks: u32,
}

impl Line {
    fn delta(&self) -> i32 {
        self.clicks as i32
            * match self.dir {
                Dir::L => -1,
                Dir::R => 1,
            }
    }
}

impl TryFrom<&str> for Line {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (head, tail) = value.split_at(1);

        let dir = match head {
            "L" => Dir::L,
            "R" => Dir::R,
            _ => return Err(format!("cannot parse dir: {}", value)),
        };

        let clicks = tail
            .parse::<u32>()
            .map_err(|err| format!("cannot parse clicks: {} ; {}", value, err))?;

        Ok(Self { dir, clicks })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn d1p1() {
        let fixture = Fixture("input.txt");
        let lines = fixture.reader().lines();

        let mut dial = Dial::new();
        let mut answer = 0;

        for line in lines.map_while(Result::ok) {
            let line = Line::try_from(line.as_str()).unwrap();

            print!("{:?} -> ", dial);
            dial.accept(&line);
            println!("{:?} :: {:?}", dial, line);

            if dial.current == 0 {
                answer += 1;
            }
        }

        println!("{}", answer);

        assert_eq!(answer, 1021);
    }
}
