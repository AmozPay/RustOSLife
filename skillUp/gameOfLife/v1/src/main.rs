mod display;
mod evolution;
mod parse;

use terminal_graphics::Display;

const MAX_X: u32 = 160;
const MAX_Y: u32 = 40;

fn main() -> Result<(), String> {
    let args = std::env::args().collect();
    let path = match parse::get_map_filepath(&args) {
        Ok(x) => x,
        Err(e) => return Err(e),
    };
    let mut map = match parse::get_map_from_filepath(path, MAX_X, MAX_Y) {
        Ok(x) => x,
        Err(e) => return Err(e),
    };

    let mut screen: Display = Display::new(MAX_X, MAX_Y);
    loop {
        display::display(&map, &mut screen);
        std::thread::sleep(std::time::Duration::new(0, 50000000));
        map = evolution::evolve(map);
    }
}
