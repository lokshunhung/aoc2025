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

struct Grid(Vec<Vec<char>>);
impl From<Lines<BufReader<File>>> for Grid {
    fn from(lines: Lines<BufReader<File>>) -> Self {
        let mut grid = Vec::new();
        for line in lines.map_while(Result::ok) {
            grid.push(line.chars().collect());
        }
        Self(grid)
    }
}
impl Grid {
    fn solve(&self) -> i32 {
        let grid = &self.0;

        let mut cnt = 0;

        for y in 0..grid.len() as isize {
            for x in 0..grid[y as usize].len() as isize {
                let cell = grid[y as usize][x as usize];
                if cell != '@' {
                    print!("{}", cell);
                    continue;
                }

                let mut adj = 0;
                for xx in (x - 1)..=(x + 1) {
                    for yy in (y - 1)..=(y + 1) {
                        if xx == x && yy == y {
                            continue;
                        }
                        if !(0 <= xx && xx < grid[y as usize].len() as isize) {
                            continue;
                        }
                        if !(0 <= yy && yy < grid.len() as isize) {
                            continue;
                        }
                        if grid[yy as usize][xx as usize] == '@' {
                            adj += 1;
                        }
                    }
                }

                if adj < 4 {
                    cnt += 1;
                    print!("x");
                } else {
                    print!("{}", cell);
                }
            }
            println!();
        }

        cnt
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn d4p1() {
        let fixture = Fixture("input.txt");
        let lines = fixture.reader().lines();
        let grid = Grid::from(lines);
        let answer = grid.solve();

        println!("{}", answer);

        assert_eq!(answer, 1395);
    }

    #[test]
    fn sample() {
        let fixture = Fixture("sample.txt");
        let lines = fixture.reader().lines();
        let grid = Grid::from(lines);
        let answer = grid.solve();

        assert_eq!(answer, 13);
    }
}
