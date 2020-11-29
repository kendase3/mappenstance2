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

// ideally of arbitrary width and height
// should have a default width of 80 and default height of 20
pub struct Mapp {
    width: u32,
    height: u32,
    mapp: Vec<Vec<Cell>>,
}

impl Mapp {
    fn new(width: u32, height: u32) -> Mapp {
        let ret = Mapp {
            width,
            height,
            mapp: Vec::new(),
        };
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
