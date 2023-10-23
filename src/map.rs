use crate::creature::Creature;
use std::sync::{Arc, Mutex};

#[derive(Clone, Copy)]
pub enum Tile {
    Wall,
    Grass,
    Water,
}

pub struct Map {
    height: usize,
    width: usize,
    creatures: Arc<Mutex<Vec<Creature>>>,

    layout: Vec<Vec<Tile>>,
}

impl Map {
    pub fn new(height: usize, width: usize, creatures: &Arc<Mutex<Vec<Creature>>>) -> Self {
        let mut layout = Vec::new();

        let mut top_row = Vec::new();
        for _ in 0..width {
            top_row.push(Tile::Wall);
        }

        layout.push(top_row);

        for _ in 1..height - 1 {
            let mut row = Vec::new();

            row.push(Tile::Wall);

            for _ in 1..(width - 1) / 2 {
                row.push(Tile::Grass);
                row.push(Tile::Water);
            }

            row.push(Tile::Wall);

            layout.push(row);
        }

        let mut bottom_row = Vec::new();
        for _ in 0..width {
            bottom_row.push(Tile::Wall);
        }

        layout.push(bottom_row);

        Map {
            height,
            width,
            creatures: creatures.clone(),
            layout,
        }
    }

    pub fn get_tile(&self, x: usize, y: usize) -> &Tile {
        &self.layout[y][x]
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_creatures(&self) {
        println!("Creatures: {}", self.creatures.lock().unwrap()[0]);
    }
}

#[readonly::make]
pub struct ReadOnlyMap {
    pub height: usize,
    pub width: usize,

    pub creatures: Vec<Creature>,
    pub layout: Vec<Vec<Tile>>,
}

impl ReadOnlyMap {
    pub fn from_map(map: &Map) -> Self {
        ReadOnlyMap {
            height: map.height,
            width: map.width,
            creatures: map
                .creatures
                .lock()
                .unwrap()
                .iter()
                .map(|e| e.clone())
                .collect(),
            layout: map
                .layout
                .iter()
                .map(|e| e.iter().map(|u| u.clone()).collect())
                .collect(),
        }
    }
}
