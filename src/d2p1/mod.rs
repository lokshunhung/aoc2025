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
            if is_invalid(n) {
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

fn is_invalid(n: u64) -> bool {
    let num_digits = n.ilog10() + 1;

    if !num_digits.is_multiple_of(2) {
        return false;
    }

    let po = 10u64.pow(num_digits / 2);
    let hi = n / po;
    let lo = n % po;

    hi == lo
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn d2p1() {
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

        assert_eq!(answer, 52316131093);
    }

    #[test]
    fn test_is_invalid() {
        assert!(is_invalid(1188511885));
    }

    #[test]
    fn test_range() {
        let range: ProductRange = "1188511880-1188511890".try_into().unwrap();
        assert_eq!(range.sum_invalids(), 1188511885);
    }
}
