use std::vec::Vec;

pub struct Cell {
    snake_frames_left: i32,
    is_snake_head: bool,
    has_food: bool,
}

pub struct Grid {
    grid: Vec<Vec<Cell>>
}
impl Clone for Cell {
    fn clone(&self) -> Self {
        Self { snake_frames_left: self.snake_frames_left.clone(), is_snake_head: self.is_snake_head.clone(), has_food: self.has_food.clone() }
    }
}
impl Cell {
    pub fn new(snake_frames_left: i32, is_snake_head: bool , has_food: bool) -> Cell {
        Cell {
            snake_frames_left: snake_frames_left,
            is_snake_head: is_snake_head,
            has_food: has_food,
        }
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
    pub fn replace(&mut self, row: usize , cell_number: usize , to_replace: Cell) {
        self.grid[row][cell_number] = to_replace
    }

    pub fn print_grid(&self) {
        const FOOD_CHAR: char = '$';
        const VOID_CHAR: char = '·';
        const SNAKE_CHAR: char = '#';
        const SNAKE_HEAD_CHAR: char = '@';
        
        for row in &self.grid {
            for cell in row {
                
                if cell.has_food {
                    print!("{}  ",FOOD_CHAR);
                }
                else if cell.is_snake_head {
                    print!("{}  ", SNAKE_HEAD_CHAR);
                }
                else if cell.snake_frames_left > 0 {
                    print!("{}  ", SNAKE_CHAR);
                }
                else {
                    print!("{}  ",VOID_CHAR);
                }
            }
            println!("");
        }
    }
}