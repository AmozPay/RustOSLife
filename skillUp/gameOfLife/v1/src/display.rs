use terminal_graphics::Colour;
use terminal_graphics::Display;

pub fn display(map: &Vec<Vec<bool>>, screen: &mut Display) {
    screen.clear();
    for (line_index, line) in map.iter().enumerate() {
        for (cell_index, cell) in line.iter().enumerate() {
            screen.set_pixel(
                cell_index as isize,
                line_index as isize,
                ' ',
                Colour::White,
                if *cell { Colour::White } else { Colour::Black },
            );
        }
    }
    screen.print();
}
