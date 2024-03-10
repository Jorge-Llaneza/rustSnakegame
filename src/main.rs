

const GAME_X: usize = 20;
const GAME_Y: usize = 20;

const DEFAULT_CELL: Cell = Cell {
    remaining_snake_frames : 0,
    has_food : false,
    
};

fn main() {
    
    
}

struct Grid {
    grid: [[Cell ; GAME_X]; GAME_X],
    
}

struct Cell {
    remaining_snake_frames: i32 ,
    has_food: bool ,
   
}

impl Grid {

    fn new() -> Grid {
        let mut grid: Grid;

        let grid_height = grid.grid.len();
        let grid_width = grid.grid[0].len();

       
        for row_index in ..grid_height {
            for cell_index in ..grid_width {
                grid.grid[grid_height][grid_width] = DEFAULT_CELL;
            }
        }
        grid
    }

}
