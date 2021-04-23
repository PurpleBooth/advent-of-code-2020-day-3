use std::io;
use std::io::BufRead;

use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("data store disconnected")]
    Disconnect(#[from] io::Error),
}

fn main() -> Result<(), Error> {
    let stdin = io::stdin();
    let mut inputs: Vec<_> = vec![];

    for line in stdin.lock().lines() {
        inputs.push(line?)
    }

    println!(
        "{:?}",
        tobogan_run(&inputs, 1, 1)
            * tobogan_run(&inputs, 1, 3)
            * tobogan_run(&inputs, 1, 5)
            * tobogan_run(&inputs, 1, 7)
            * tobogan_run(&inputs, 2, 1)
    );

    Ok(())
}

#[derive(Debug, Eq, PartialEq)]
struct Tobogan {
    y: usize,
    x: usize,
    width: usize,
    down: usize,
    right: usize,
}

impl Tobogan {
    fn new(width: usize, down: usize, right: usize) -> Tobogan {
        Tobogan {
            y: 0,
            x: 0,
            width,
            down,
            right,
        }
    }

    fn slide(&self) -> Tobogan {
        Tobogan {
            y: self.y + self.down,
            x: (self.x + self.right) % self.width,
            width: self.width,
            down: self.down,
            right: self.right,
        }
    }

    fn index(&self) -> usize {
        ((self.y * self.width) + self.x) as usize
    }
}

fn tobogan_run(map: &[String], down: usize, right: usize) -> usize {
    let first_row = map.get(0);
    if first_row.is_none() {
        return 0;
    }

    let width = first_row.unwrap().len();

    map.iter()
        .flat_map(|row| row.chars())
        .enumerate()
        .fold(
            (0, Tobogan::new(width, down, right)),
            |(trees_hit, tobogan), (char_index, character)| match (
                tobogan.index(),
                char_index,
                character,
            ) {
                (index, char_index, '#') if char_index == index => (trees_hit + 1, tobogan.slide()),
                (index, chat_index, '.') if chat_index == index => (trees_hit, tobogan.slide()),
                (_, _, _) => (trees_hit, tobogan),
            },
        )
        .0
}

#[cfg(test)]
mod tests {
    use crate::{tobogan_run, Tobogan};

    #[test]
    fn tobogan_index() {
        assert_eq!(
            3,
            Tobogan {
                down: 1,
                right: 3,
                y: 1,
                x: 1,
                width: 2,
            }
            .index()
        );
    }

    #[test]
    fn tobogan_slide() {
        let mut tobogan = Tobogan {
            x: 0,
            y: 0,
            width: 8,
            down: 1,
            right: 3,
        }
        .slide();
        assert_eq!(
            Tobogan {
                y: 1,
                x: 3,
                width: 8,
                down: 1,
                right: 3
            },
            tobogan
        );
        assert_eq!(8 + 3, tobogan.index());
        tobogan = tobogan.slide();
        assert_eq!(
            Tobogan {
                y: 2,
                x: 6,
                width: 8,
                down: 1,
                right: 3
            },
            tobogan
        );
        assert_eq!((2 * 8) + 6, tobogan.index());
        tobogan = tobogan.slide();
        assert_eq!(
            Tobogan {
                y: 3,
                x: 1,
                width: 8,
                down: 1,
                right: 3
            },
            tobogan
        );
        assert_eq!((3 * 8) + 1, tobogan.index());
    }

    #[test]
    fn no_input_is_0() {
        assert_eq!(0, tobogan_run(&[], 1, 3))
    }

    #[test]
    fn single_tree() {
        assert_eq!(1, tobogan_run(&["#".into()], 1, 3))
    }

    #[test]
    fn open_field() {
        assert_eq!(0, tobogan_run(&["....".into(), "....".into()], 1, 3))
    }

    #[test]
    fn one_tree_hill() {
        assert_eq!(1, tobogan_run(&["....".into(), "...#".into()], 1, 3))
    }

    #[test]
    fn given_example() {
        assert_eq!(
            7,
            tobogan_run(
                &[
                    "..##.......".into(),
                    "#...#...#..".into(),
                    ".#....#..#.".into(),
                    "..#.#...#.#".into(),
                    ".#...##..#.".into(),
                    "..#.##.....".into(),
                    ".#.#.#....#".into(),
                    ".#........#".into(),
                    "#.##...#...".into(),
                    "#...##....#".into(),
                    ".#..#...#.#".into(),
                ],
                1,
                3
            )
        )
    }
}
