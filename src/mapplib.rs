// TODO(kendall): make a mapp class that's printable etc.

use std::fmt;

pub struct Cell {
    pub c: char,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.c)
    }
}
