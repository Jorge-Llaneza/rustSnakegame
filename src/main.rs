mod grid;
mod game;



fn main() {
    let food_cell: grid::Cell = grid::Cell::new(0, false, true);
    let mut me = grid::Grid::new_grid(20 , 20);
    
    //me.print_grid();
    me.replace(4, 5, food_cell);
    me.print_grid();

}

