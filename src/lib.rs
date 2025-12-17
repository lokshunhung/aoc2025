#![allow(dead_code, unused)]

use std::fs;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Lines;
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

struct Input;
impl TryFrom<&mut Lines<BufReader<File>>> for Input {
    type Error = String;

    fn try_from(lines: &mut Lines<BufReader<File>>) -> Result<Self, Self::Error> {
        for line in lines.map_while(Result::ok) {
            //
        }

        Ok(Input)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let fixture = Fixture("input.txt");
        let mut lines = fixture.reader().lines();
        let input = Input::try_from(&mut lines).unwrap();
    }
}

pub mod d1p1;
pub mod d1p2;
pub mod d2p1;
pub mod d2p2;
pub mod d3p1;
pub mod d3p2;
pub mod d4p1;
pub mod d4p2;
pub mod d5p1;
pub mod d5p2;
