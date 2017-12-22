use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::collections::HashSet;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Coord((i32, i32));
impl From<(i32, i32)> for Coord {
    fn from(t: (i32, i32)) -> Coord {
        Coord((t.0,t.1))
    }
}
#[derive(Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn(&self, d: Direction) -> Direction {
        match d {
            Direction::Left => {
                match self {
                    &Direction::Up => Direction::Left,
                    &Direction::Down => Direction::Right,
                    &Direction::Left => Direction::Down,
                    &Direction::Right => Direction::Up
                }
            },
            Direction::Right => {
                match self {
                    &Direction::Up => Direction::Right,
                    &Direction::Down => Direction::Left,
                    &Direction::Left => Direction::Up,
                    &Direction::Right => Direction::Down
                }
            },
            _ => panic!("Cannot turn up or down")
        }
    }
}

impl Coord {
    fn go(&mut self, d: &Direction) {
        match d {
            &Direction::Up => (self.0).1 -= 1,
            &Direction::Down => (self.0).1 += 1,
            &Direction::Left => (self.0).0 -= 1,
            &Direction::Right => (self.0).0 += 1
        }
    }
    
}

#[derive(Debug)]
struct Grid(HashSet<Coord>);

impl Grid {
    fn from_file<P: AsRef<Path>>(p: P) -> Result<Grid, String> {
        let f = File::open(p).map_err(|e| format!("{}", e))?;
        Ok(Grid(BufReader::new(f).lines().enumerate()
             .flat_map(|(row, line)| {
                 line.unwrap()
                     .chars().enumerate()
                     .filter(|s| s.1 == '#')
                     .map(|(col, _)| (col as i32, row as i32).into())
                     .collect::<Vec<Coord>>()
             }).collect()))
    }

    fn infect(&mut self, n: usize) -> u32 {
        let mut pos = self.middle();
        let mut direction = Direction::Up;
        let mut infected = 0;
        for _ in 0..n {
            if self.0.remove(&pos) {
                direction = direction.turn(Direction::Right);
            } else {
                direction = direction.turn(Direction::Left);
                self.0.insert(pos.clone());
                infected +=1;
            }
            pos.go(&direction);
        }
        infected
    }

    fn middle(&self) -> Coord {
        let m = self.0.iter().map(|&Coord((x,y))| std::cmp::max(x, y)).max().unwrap();
        debug_assert!(m % 2 == 0);
        let c = m / 2;
        Coord((c, c))
    }


}


fn main() {
    let mut grid = Grid::from_file("input.txt").unwrap();
    println!("22a: {} have been infected during 10K rounds", grid.infect(10_000));
}

#[test]
fn test() {
    let mut grid = Grid::from_file("test.txt").unwrap();
    assert_eq!(grid.infect(10_000),5587);
}
