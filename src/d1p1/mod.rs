#![allow(dead_code)]

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
    use std::fs;
    use std::io;
    use std::io::BufRead;
    use std::path;

    use super::*;

    #[test]
    fn d1p1() {
        let pb = path::Path::new(file!()).parent().unwrap().join("input.txt");
        let f = fs::File::open(&pb).unwrap();
        let br = io::BufReader::new(f).lines();

        let mut dial = Dial::new();
        let mut answer = 0;

        for line in br.map_while(Result::ok) {
            let line = line.as_str().try_into().unwrap();

            print!("{:?} -> ", dial);
            dial.accept(&line);
            println!("{:?} :: {:?}", dial, line);

            if dial.current == 0 {
                answer += 1;
            }
        }

        println!("{}", answer);
    }
}
