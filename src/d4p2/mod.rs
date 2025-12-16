#![allow(dead_code, unused)]

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

fn solve_once(grid: &mut [Vec<char>], cont: &mut bool) -> i32 {
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
                grid[y as usize][x as usize] = 'x';
                *cont = true;
            } else {
                print!("{}", cell);
            }
        }
        println!();
    }

    cnt
}

fn solve(grid: &mut [Vec<char>]) -> i32 {
    let mut answer = 0;
    let mut cont: bool;
    loop {
        cont = false;
        answer += solve_once(grid, &mut cont);
        if !cont {
            break;
        }
        println!();
    }
    answer
}

#[cfg(test)]
mod test {
    use std::fs;
    use std::io;
    use std::io::BufRead;
    use std::path;

    use super::*;

    #[test]
    fn d4p2() {
        let mut grid = parse("input.txt");
        let answer = solve(&mut grid);

        println!("{}", answer);
    }

    #[test]
    fn sample() {
        let mut grid = parse("sample.txt");
        let answer = solve(&mut grid);

        assert_eq!(answer, 43);
    }
}
