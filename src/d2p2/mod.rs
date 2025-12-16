#![allow(dead_code)]

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
    use std::fs;
    use std::io;
    use std::io::BufRead;
    use std::path;

    use super::*;

    #[test]
    fn d2p2() {
        let pb = path::Path::new(file!()).parent().unwrap().join("input.txt");
        let f = fs::File::open(&pb).unwrap();
        let br = io::BufReader::new(f).lines();

        let mut answer = 0;

        for line in br.map_while(Result::ok) {
            for item in line.split(",") {
                let pr = ProductRange::try_from(item).unwrap();
                println!("{:?}", pr);
                answer += pr.sum_invalids();
            }
        }

        println!("{}", answer);
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
