use super::cell::Cell;
use crate::ui::{PADDING_H, PADDING_W};

pub type WorldMap = Vec<Vec<Cell>>;

pub struct World {
    pub map: WorldMap,
    rows: u16,
    cols: u16,
}

impl World {
    pub fn new(rows: u16, cols: u16) -> World {
        if rows <= PADDING_W * 2 || cols <= PADDING_H * 2 {
            panic!("The terminal is too small, please resize it.");
        }

        let rows = rows - PADDING_W;
        let cols = cols / 2 - PADDING_H;
        World {
            map: vec![vec![Cell::Dead; cols as usize]; rows as usize],
            rows,
            cols,
        }
    }

    pub fn random_map(&mut self, poblation: f32) {
        let cant_cells = ((self.rows * self.cols) as f32 * poblation) as usize;
        for _ in 0..cant_cells {
            let x = rand::random::<usize>() % self.map.len();
            let y = rand::random::<usize>() % self.map[0].len();
            self.map[x][y] = Cell::Alive;
        }
    }

    pub fn next_iteration(&mut self) {
        let mut new_map = self.map.clone();

        for (i, row) in self.map.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                let alive_neighbors = self.get_neighbors_count(i as i32, j as i32);

                if cell.is_alive() {
                    if alive_neighbors < 2 || alive_neighbors > 3 {
                        new_map[i][j] = Cell::Dead;
                    }
                } else {
                    if alive_neighbors == 3 {
                        new_map[i][j] = Cell::Alive;
                    }
                }
            }
        }

        self.map = new_map;
    }

    fn get_neighbors_count(&self, row: i32, col: i32) -> u8 {
        let mut count = 0;

        for x in -1..=1 {
            for y in -1..=1 {
                if x == 0 && y == 0 {
                    continue;
                }

                let new_i = row + x;
                let new_j = col + y;

                if self.is_cell_alive(new_i, new_j) {
                    count += 1;
                }
            }
        }

        count
    }

    pub fn is_cell_alive(&self, row: i32, col: i32) -> bool {
        if row < 0 || row >= self.map.len() as i32 {
            return false;
        }

        if col < 0 || col >= self.map[0].len() as i32 {
            return false;
        }

        self.map[row as usize][col as usize].is_alive()
    }
}
