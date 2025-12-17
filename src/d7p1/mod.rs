#![allow(dead_code, unused)]

use std::collections::HashSet;
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

struct Problem {
    itr: Box<dyn Iterator<Item = String>>,
}
impl From<BufReader<File>> for Problem {
    fn from(reader: BufReader<File>) -> Self {
        let itr = reader.lines().map_while(Result::ok);
        Problem { itr: Box::new(itr) }
    }
}
impl Problem {
    fn solve(&mut self) -> u32 {
        let mut cnt = 0;
        let (mut beams, bounds) = {
            let first_line = self.itr.next().unwrap();
            let bounds = 0..(first_line.len());
            let beam_pos = first_line.find('S').unwrap();
            let mut set = HashSet::new();
            set.insert(beam_pos);
            (set, bounds)
        };

        for line in &mut self.itr {
            let mut new_beams = HashSet::<usize>::new();

            let splitter_pos = line
                .chars()
                .enumerate()
                .filter_map(|(i, c)| match c {
                    '^' => Some(i),
                    '.' => None,
                    _ => panic!(),
                })
                .collect::<HashSet<_>>();

            for pos in beams {
                if !splitter_pos.contains(&pos) {
                    new_beams.insert(pos);
                    continue;
                }

                cnt += 1;

                if pos > bounds.start {
                    new_beams.insert(pos - 1);
                }

                if pos < bounds.end - 1 {
                    new_beams.insert(pos + 1);
                }
            }

            beams = new_beams;
        }

        cnt
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn d7p1() {
        let fixture = Fixture("input.txt");
        let reader = fixture.reader();
        let mut problem = Problem::from(reader);
        let answer = problem.solve();

        println!("{}", answer);

        assert_eq!(answer, 1717);
    }

    #[test]
    fn sample() {
        let fixture = Fixture("sample.txt");
        let reader = fixture.reader();
        let mut problem = Problem::from(reader);
        let answer = problem.solve();

        assert_eq!(answer, 21);
    }
}
