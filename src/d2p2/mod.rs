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
struct ProductRange {
    lo: u64,
    hi: u64,
}

impl ProductRange {
    fn sum_invalids(&self) -> u64 {
        let mut sum = 0;
        for n in self.lo..=self.hi {
            if n.is_invalid() {
                sum += n;
            }
        }
        sum
    }
}

impl TryFrom<&str> for ProductRange {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut i = value.split("-");
        let lo = i
            .next()
            .ok_or_else(|| format!("invalid line {}", value))?
            .parse::<u64>()
            .map_err(|err| format!("failed to parse lo '{}': {}", value, err))?;
        let hi = i
            .next()
            .ok_or_else(|| format!("invalid line {}", value))?
            .parse::<u64>()
            .map_err(|err| format!("failed to parse hi '{}': {}", value, err))?;
        Ok(Self { lo, hi })
    }
}

trait IsInvalidProductId {
    fn is_invalid(&self) -> bool;
}

impl IsInvalidProductId for u64 {
    fn is_invalid(&self) -> bool {
        let num_digits = self.ilog10() + 1;
        for exp in 1..=num_digits / 2 {
            if num_digits.is_multiple_of(exp) {
                let po = 10u64.pow(exp);

                let base = self % po;
                let mut running = self / po;

                loop {
                    if running % po != base {
                        break;
                    }
                    running /= po;
                    if running == 0 {
                        return true;
                    }
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn d2p2() {
        let fixture = Fixture("input.txt");
        let lines = fixture.reader().lines();

        let mut answer = 0;

        for line in lines.map_while(Result::ok) {
            for item in line.split(",") {
                let pr = ProductRange::try_from(item).unwrap();
                println!("{:?}", pr);
                answer += pr.sum_invalids();
            }
        }

        println!("{}", answer);

        assert_eq!(answer, 69564213293);
    }

    #[test]
    fn test_valid() {
        assert!(!1u64.is_invalid());
        assert!(!12u64.is_invalid());
    }

    #[test]
    fn test_is_invalid() {
        assert!(11u64.is_invalid());
        assert!(123123u64.is_invalid());
        assert!(123123123u64.is_invalid());
    }
}
