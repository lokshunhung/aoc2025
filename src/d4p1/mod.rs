#![allow(dead_code)]

use std::fs;
use std::io;
use std::io::BufRead;
use std::path;

fn parse(path: impl AsRef<std::path::Path>) -> Vec<Vec<char>> {
    let pb = path::Path::new(file!()).parent().unwrap().join(path);
    let f = fs::File::open(&pb).unwrap();
    let br = io::BufReader::new(f).lines();

    let mut grid = Vec::<Vec<char>>::new();

    for line in br.map_while(Result::ok) {
        grid.push(line.chars().collect());
    }

    grid
}

fn solve(grid: &[Vec<char>]) -> i32 {
    let mut cnt = 0;

    for y in 0..grid.len() as isize {
        let line = &grid[y as usize];
        for x in 0..line.len() as isize {
            let cell = line[x as usize];
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
                    if !(0 <= xx && xx < line.len() as isize) {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn d4p1() {
        let grid = parse("input.txt");
        let cnt = solve(&grid);
        println!("{}", cnt);
    }

    #[test]
    fn sample() {
        let grid = parse("sample.txt");
        let cnt = solve(&grid);

        assert_eq!(cnt, 13);
    }
}
