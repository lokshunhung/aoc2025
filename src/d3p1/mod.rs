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
struct Bank(Vec<u8>);
impl Bank {
    fn get_largest(&self) -> u64 {
        let mut m1 = self.0.len() - 2;
        let mut m2 = self.0.len() - 1;
        for i in (0..self.0.len() - 2).rev() {
            let n = self.0[i];
            if n >= self.0[m1] {
                if self.0[m1] >= self.0[m2] {
                    m2 = m1;
                }
                m1 = i;
            }
        }
        self.0[m1] as u64 * 10 + self.0[m2] as u64
    }
}
impl TryFrom<&str> for Bank {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let vec = value
            .chars()
            .map(|c| c.to_digit(10).map(|d| d as u8))
            .collect::<Option<Vec<_>>>()
            .ok_or_else(|| format!("cannot parse {}", value))?;
        Ok(Self(vec))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn d3p1() {
        let fixture = Fixture("input.txt");
        let lines = fixture.reader().lines();

        let mut answer = 0;

        for line in lines.map_while(Result::ok) {
            let bank = Bank::try_from(line.as_str()).unwrap();
            answer += bank.get_largest();
        }

        println!("{}", answer);

        assert_eq!(answer, 17452);
    }

    #[test]
    fn sanity() {
        assert_eq!(Bank(vec![1, 2, 3, 4]).get_largest(), 34);
        assert_eq!(Bank(vec![9, 1, 1, 1, 2]).get_largest(), 92);
    }

    #[test]
    fn from_example() {
        assert_eq!(Bank::try_from("987654321111111").unwrap().get_largest(), 98);
        assert_eq!(Bank::try_from("811111111111119").unwrap().get_largest(), 89);
        assert_eq!(Bank::try_from("234234234234278").unwrap().get_largest(), 78);
        assert_eq!(Bank::try_from("818181911112111").unwrap().get_largest(), 92);
    }
}
