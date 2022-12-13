mod cell;
use cell::Cell;

struct Grid {
    width: i32,
    height: i32,
    cells: Vec<Cell>,
}
