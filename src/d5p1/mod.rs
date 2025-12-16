#![allow(dead_code, unused)]

use std::fs;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Lines;
use std::ops::RangeInclusive;
use std::path;
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
struct FreshIngredients {
    ranges: Vec<RangeInclusive<u64>>,
}
impl TryFrom<&mut Lines<BufReader<File>>> for FreshIngredients {
    type Error = String;

    fn try_from(lines: &mut Lines<BufReader<File>>) -> Result<Self, Self::Error> {
        let mut ranges = Vec::new();
        for line in lines.map_while(Result::ok) {
            if line.is_empty() {
                break;
            }

            let mut s = line.split("-");
            let lo = s
                .next()
                .ok_or_else(|| format!("cannot parse line {}", line))?
                .parse::<u64>()
                .map_err(|err| format!("cannot parse line {} err: {}", line, err))?;
            let hi = s
                .next()
                .ok_or_else(|| format!("cannot parse line {}", line))?
                .parse::<u64>()
                .map_err(|err| format!("cannot parse line {} err: {}", line, err))?;
            ranges.push(lo..=hi);
        }
        Ok(Self { ranges })
    }
}
impl FreshIngredients {
    fn includes(&self, id: u64) -> bool {
        for range in &self.ranges {
            if range.contains(&id) {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn d5p1() {
        let fixture = Fixture("input.txt");
        let mut lines = fixture.reader().lines();

        let fresh = FreshIngredients::try_from(&mut lines).unwrap();
        println!("{:#?}", fresh);

        let mut answer = 0;

        for line in lines.map_while(Result::ok) {
            let id = line.parse::<u64>().unwrap();

            if fresh.includes(id) {
                answer += 1;
            }
        }

        println!("{}", answer);
    }

    #[test]
    fn sample() {
        let fixture = Fixture("sample.txt");
        let mut lines = fixture.reader().lines();

        let fresh = FreshIngredients::try_from(&mut lines).unwrap();
        println!("{:#?}", fresh);

        let mut answer = 0;

        for line in lines.map_while(Result::ok) {
            let id = line.parse::<u64>().unwrap();

            if fresh.includes(id) {
                answer += 1;
            }
        }

        assert_eq!(answer, 3);
    }
}
