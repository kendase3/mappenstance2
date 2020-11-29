use mapplib::Cell;
use mapplib::Mapp;

fn main() {
    let cell = Cell { c: 'a' };
    println!("cell = {}", cell);
    let mapp = Mapp::default();
    println!("mapp:\n{}", mapp);
}
