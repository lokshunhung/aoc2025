#![allow(dead_code)]

#[derive(Debug)]
struct Joltage([usize; 12]);

impl From<&Bank> for Joltage {
    fn from(value: &Bank) -> Self {
        let len = value.0.len();
        let a = std::array::from_fn(move |i| len - 12 + i);
        Self(a)
    }
}

impl Joltage {
    fn step(&mut self, bank: &Bank, i: usize) {
        let mut i = i;
        for j in 0..12 {
            let value = bank.0[i];
            let current_index = self.0[j];
            let current_value = bank.0[current_index];
            if value >= current_value {
                self.0[j] = i;
                i = current_index;
            } else {
                break;
            }
        }
    }

    fn value(&self, bank: &Bank) -> u64 {
        self.0.iter().fold(0, |acc, cur| acc * 10 + bank.0[*cur] as u64)
    }
}

#[derive(Debug)]
struct Bank(Vec<u8>);

impl Bank {
    fn get_largest(&self) -> u64 {
        let mut jol = Joltage::from(self);
        for i in (0..self.0.len() - 12).rev() {
            jol.step(self, i);
        }
        jol.value(self)
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
    use std::fs;
    use std::io;
    use std::io::BufRead;
    use std::path;

    use super::*;

    #[test]
    fn test() {
        let pb = path::Path::new(file!()).parent().unwrap().join("input.txt");
        let f = fs::File::open(&pb).unwrap();
        let br = io::BufReader::new(f).lines();

        let mut answer = 0;

        for line in br.map_while(Result::ok) {
            let bank = Bank::try_from(line.as_str()).unwrap();
            answer += bank.get_largest();
        }

        println!("{}", answer);
    }

    #[test]
    fn joltage_from() {
        let j = Joltage::from(&Bank(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]));
        assert_eq!(j.0, [4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
    }

    #[test]
    fn from_example() {
        assert_eq!(Bank::try_from("987654321111111").unwrap().get_largest(), 987654321111);
        assert_eq!(Bank::try_from("811111111111119").unwrap().get_largest(), 811111111119);
        assert_eq!(Bank::try_from("234234234234278").unwrap().get_largest(), 434234234278);
        assert_eq!(Bank::try_from("818181911112111").unwrap().get_largest(), 888911112111);
    }
}
