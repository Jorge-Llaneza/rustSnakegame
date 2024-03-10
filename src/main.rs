const GAME_X: usize = 20;
const GAME_Y: usize = 20;

const DEFAULT_CELL: Cell = {
    remaining_snake_frames = 0;
    has_food = false;
};

fn main() {

    
}

struct Grid<T> {
    grid: [[T ; GAME_X]; GAME_X],
    
}

struct Cell {
    remaining_snake_frames: i32 ,
    has_food: bool ,
    is_snake_head: bool,
}

impl Grid<T> {

    fn new() -> Grid<Cell> {
        for

    }

}
