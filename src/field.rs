#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Cell
{
    Empty,
    X,
    O
}

#[derive(Debug, PartialEq)]
pub struct Field
{
    size: usize,
    cells: Vec<Cell>
}

impl Field
{
    pub fn new(size: usize) -> Field {
        Field {
            size,
            cells: vec![Cell::Empty; size * size]
        }
    }

    pub fn set(&mut self, x: usize, y: usize, cell: Cell)  {
        assert!(x < self.size);
        assert!(y < self.size);
        assert!(cell != Cell::Empty);

        self.cells[x * self.size + y] = cell;
    }

    pub fn get(&self, x: usize, y: usize) -> Cell {
        assert!(x < self.size);
        assert!(y < self.size);

        self.cells[x * self.size + y]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn field_new() {
        let field = Field::new(3);

        assert_eq!(3, field.size);
        assert_eq!(vec![Cell::Empty; 9], field.cells);
    }

    #[test]
    fn field_get_new() {
        let field = Field::new(3);

        for x in 0..3 {
            for y in 0..3 {
                assert_eq!(Cell::Empty, field.get(x, y));
            }
        }
    }

    #[test]
    fn field_set() {
        let mut field = Field::new(3);

        field.set(0, 0, Cell::X);
        field.set(1, 1, Cell::O);
        field.set(2, 2, Cell::X);
        field.set(0, 2, Cell::O);
        field.set(2, 0, Cell::X);

        assert_eq!(vec![Cell::X, Cell::Empty, Cell::O, Cell::Empty, Cell::O, Cell::Empty, Cell::X, Cell::Empty, Cell::X], field.cells);
    }

    #[test]
    fn field_set_get() {
        let mut field = Field::new(3);

        field.set(0, 0, Cell::X);
        assert_eq!(Cell::X, field.get(0, 0));
        field.set(1, 1, Cell::O);
        assert_eq!(Cell::O, field.get(1, 1));
        field.set(2, 2, Cell::X);
        assert_eq!(Cell::X, field.get(2, 2));
        field.set(0, 2, Cell::O);
        assert_eq!(Cell::O, field.get(0, 2));
        field.set(2, 0, Cell::X);
        assert_eq!(Cell::X, field.get(2, 0));
    }
}