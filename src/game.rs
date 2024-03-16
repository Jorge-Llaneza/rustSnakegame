
use rand::Rng;

use crate::grid;
extern crate rand;




pub fn game_init(grid: &mut grid::Grid) {
    
    let snake_row = grid.get_height()/2 ;
    let snake_cell = grid.get_width()/2;


}

fn add_food(grid: &mut grid::Grid) {
    
    loop {
        let food_cell = grid::Cell::new(0, false, true);
        let rand_height = rand::thread_rng()
            .gen_range(1..grid.get_height());
        let rand_width = rand::thread_rng()
        .gen_range(1..grid.get_width());
    
        let cell = grid.get(rand_height, rand_width);

        if !cell.is_snake_head {
            if cell.snake_frames_left > 0 {
                grid.replace(rand_height, rand_width, food_cell)
            }
        }
    }
}
