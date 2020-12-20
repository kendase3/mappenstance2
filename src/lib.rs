// TODO(kendall): make a mapp class that's printable etc.

use std::fmt;

pub struct Cell {
    pub c: char,
}

impl Cell {
    fn new() -> Cell {
        Cell {
            c: Cell::get_default_char(),
        }
    }
    fn get_default_char() -> char {
        '_'
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.c)
    }
}

// ideally of arbitrary width and height
// should have a default width of 80 and default height of 20
pub struct Mapp {
    mapp: Vec<Vec<Cell>>,
}

impl Mapp {
    fn new(width: u32, height: u32) -> Mapp {
        let mut ret = Mapp { mapp: Vec::new() };
        for _ in 0..height {
            let mut row: Vec<Cell> = Vec::new();
            for _ in 0..width {
                row.push(Cell::new());
            }
            ret.mapp.push(row);
        }
        ret
    }
    fn get_default_height() -> u32 {
        20
    }
    fn get_default_width() -> u32 {
        80
    }
}

impl Default for Mapp {
    fn default() -> Mapp {
        Mapp::new(Mapp::get_default_width(), Mapp::get_default_height())
    }
}

impl fmt::Display for Mapp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // iterate over rows
        for row in &self.mapp {
            for cell in row {
                write!(f, "{}", cell.c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
