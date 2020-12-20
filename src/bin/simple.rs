use mappenstance2::{Cell, Mapp};

fn main() {
    let cell = Cell { c: 'a' };
    println!("cell = {}", cell);
    let mapp = Mapp::default();
    println!("mapp:\n{}", mapp);
}
