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

    fn accept(&mut self, line: &Line) -> u32 {
        let mut inc = line.clicks.div_euclid(100);
        let line = Line {
            dir: line.dir.clone(),
            clicks: line.clicks.rem_euclid(100),
        };
        let sum = self.current + line.delta();
        if self.current != 0 {
            match line.dir {
                Dir::L => {
                    if sum <= 0 {
                        inc += 1;
                    }
                }
                Dir::R => {
                    if sum >= 100 {
                        inc += 1;
                    }
                }
            }
        }
        self.current = sum.rem_euclid(100);
        inc
    }
}

#[derive(Clone, Debug, PartialEq)]
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
    fn d1p2() {
        let fixture = Fixture("input.txt");
        let lines = fixture.reader().lines();

        let mut dial = Dial::new();
        let mut answer = 0;

        for line in lines.map_while(Result::ok) {
            let line = line.as_str().try_into().unwrap();

            print!("{:?} -> ", dial);
            let inc = dial.accept(&line);
            println!("{:?} :: {:?} :: {}", dial, line, inc);

            answer += inc;
        }

        println!("{}", answer);

        assert_eq!(answer, 5933);
    }

    #[test]
    fn example_inc() {
        let mut dial = Dial { current: 50 };
        let mut inc: u32;

        inc = dial.accept(&"L68".try_into().unwrap());
        assert_eq!((dial.current, inc), (82, 1));

        inc = dial.accept(&"L30".try_into().unwrap());
        assert_eq!((dial.current, inc), (52, 0));

        inc = dial.accept(&"R48".try_into().unwrap());
        assert_eq!((dial.current, inc), (0, 1));

        inc = dial.accept(&"L5".try_into().unwrap());
        assert_eq!((dial.current, inc), (95, 0));

        inc = dial.accept(&"R60".try_into().unwrap());
        assert_eq!((dial.current, inc), (55, 1));

        inc = dial.accept(&"L55".try_into().unwrap());
        assert_eq!((dial.current, inc), (0, 1));
    }

    #[test]
    fn careful_inc() {
        let mut dial = Dial { current: 50 };
        let inc = dial.accept(&"R1000".try_into().unwrap());
        assert_eq!(dial.current, 50);
        assert_eq!(inc, 10);
    }

    #[test]
    fn edge_case1() {
        let mut dial = Dial { current: 52 };
        let line = Line::try_from("R48").unwrap();
        let inc = dial.accept(&line);
        assert_eq!((dial.current, inc), (0, 1));
    }

    #[test]
    fn edge_case2() {
        let mut dial = Dial { current: 0 };
        let line = Line::try_from("L5").unwrap();
        let inc = dial.accept(&line);
        assert_eq!((dial.current, inc), (95, 0));
    }
}
