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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_should_have_a_constructor() {
        let cell = Cell::new(0, 0);
        assert_eq!(cell.x, 0);
        assert_eq!(cell.y, 0);
    }

    fn get_x_as_f64_should_return_x_as_f64() {
        let cell = Cell::new(1, 0);
        assert_eq!(cell.get_x_as_f64(), 1.0);
    }

    fn get_y_as_f64_should_return_y_as_f64() {
        let cell = Cell::new(0, 2);
        assert_eq!(cell.get_y_as_f64(), 2.0);
    }
}
