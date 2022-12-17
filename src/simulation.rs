use std::collections::HashMap;

use crate::{Block, Cave, Pos};
use generator::{done, Generator, Gn};

pub type Simulation<'a> = Generator<'a, Step, Step>;

#[derive(Debug, Clone)]
pub struct Step {
    pub tiles: HashMap<Pos, Block>,
    pub grain: Option<Pos>,
}

pub fn part1(mut cave: Cave) -> Simulation<'static> {
    Gn::new_scoped(move |mut s| {
        let abyss_line = cave.tiles.keys().map(|pos| pos.y).max().unwrap() + 1;

        loop {
            // create_frame(&cave, None, frame_num);
            s.yield_(Step {
                tiles: cave.tiles.clone(),
                grain: None,
            });
            let mut grain_moves = cave.next_grain();
            for pos in grain_moves.by_ref() {
                s.yield_(Step {
                    tiles: cave.tiles.clone(),
                    grain: Some(pos.clone()),
                });
                // create_frame(&cave, Some(pos.clone()), frame_num);
                if pos.y > abyss_line {
                    done!();
                }
            }
            cave.tiles.insert(grain_moves.pos, Block::Sand);
        }
    })
}
