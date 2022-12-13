#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Cell {
    pub x: i32,
    pub y: i32,
}
impl Cell {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    pub(crate) fn get_x_as_f64(&self) -> f64 {
        self.x as f64
    }
    pub(crate) fn get_y_as_f64(&self) -> f64 {
        self.y as f64
    }
}
