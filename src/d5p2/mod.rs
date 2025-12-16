#![allow(dead_code, unused)]

use std::cmp::Ordering;
use std::cmp::max;
use std::collections::HashSet;
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
    fn fix_range(self) -> SortedFreshIngredients {
        let mut ranges = self.ranges;
        ranges.sort_by(|a, b| a.start().cmp(b.start()).then_with(|| a.end().cmp(b.end())));

        let mut result = vec![ranges[0].clone()];

        for range in ranges.iter().skip(1) {
            let prev = result.last().unwrap();
            let curr = range;

            let lo = max(prev.end() + 1, *curr.start());
            let hi = *curr.end();

            if lo > hi {
                continue;
            }

            result.push(lo..=hi);
        }

        SortedFreshIngredients { ranges: result }
    }
}

#[derive(Debug)]
struct SortedFreshIngredients {
    ranges: Vec<RangeInclusive<u64>>,
}
impl SortedFreshIngredients {
    fn fresh_count(&self) -> u64 {
        self.ranges
            .iter()
            .fold(0, |acc, cur| acc + (*cur.end()) - (*cur.start()) + 1)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn d5p2() {
        let fixture = Fixture("input.txt");
        let mut lines = fixture.reader().lines();

        let fresh = FreshIngredients::try_from(&mut lines).unwrap();
        println!("{:#?}", fresh);

        let sorted = fresh.fix_range();
        println!("{:#?}", sorted);

        let answer = sorted.fresh_count();

        println!("{}", answer);
    }

    #[test]
    fn sample() {
        let fixture = Fixture("sample.txt");
        let mut lines = fixture.reader().lines();

        let mut fresh = FreshIngredients::try_from(&mut lines).unwrap();
        println!("{:#?}", fresh);

        let sorted = fresh.fix_range();
        println!("{:#?}", sorted);

        let answer = sorted.fresh_count();

        assert_eq!(answer, 14);
    }
}
