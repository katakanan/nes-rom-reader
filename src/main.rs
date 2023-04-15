extern crate image;
extern crate piston_window;

mod cassette;

use cassette::Cassette;

fn main() {
    println!("Hello World");

    let cassette = Cassette::load("../nes-roms/sample1.nes").unwrap();
    cassette.show();
}
