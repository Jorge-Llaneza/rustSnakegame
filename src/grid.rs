use std::vec::Vec;

struct Cell {
    snake_frames_left: i32,
    is_snake_head: bool,
    has_food: bool,
}

struct Grid {
    grid: Vec<Vec<Cell>>
}
impl Clone for Cell {
    fn clone(&self) -> Self {
        Self { snake_frames_left: self.snake_frames_left.clone(), is_snake_head: self.is_snake_head.clone(), has_food: self.has_food.clone() }
    }
}

impl Grid {

    pub fn new_grid(width: usize, height: usize) -> Grid {

        const DEFAULT_CELL: Cell = Cell {
            snake_frames_left: 0,
            is_snake_head: false,
            has_food: false,
        }; 
        Grid {
            grid: vec![vec![DEFAULT_CELL ; width] ; height]
        }

    }
}