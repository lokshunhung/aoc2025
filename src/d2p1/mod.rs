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

    if num_digits % 2 == 1 {
        return false;
    }

    let po = 10u64.pow(num_digits / 2);
    let hi = n / po;
    let lo = n % po;

    hi == lo
}

#[cfg(test)]
mod test {
    use std::fs;
    use std::io;
    use std::io::BufRead;
    use std::path;

    use super::*;

    #[test]
    fn d2p1() {
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
    fn test_is_invalid() {
        assert!(is_invalid(1188511885));
    }

    #[test]
    fn test_range() {
        let range: ProductRange = "1188511880-1188511890".try_into().unwrap();
        assert_eq!(range.sum_invalids(), 1188511885);
    }
}
