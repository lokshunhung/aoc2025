#![allow(dead_code, unused)]

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

        for line in br.map_while(Result::ok) {
            //
        }
    }
}

pub mod d1p1;
pub mod d1p2;
pub mod d2p1;
pub mod d2p2;
pub mod d3p1;
pub mod d3p2;
pub mod d4p1;
