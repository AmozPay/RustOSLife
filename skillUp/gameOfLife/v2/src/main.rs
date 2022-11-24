//#![deny(warnings)]

use speedy2d::color::Color;
use speedy2d::dimen::{UVec2, Vec2};
use speedy2d::window::{MouseButton, WindowHandler, WindowHelper, WindowStartupInfo};
use speedy2d::{Graphics2D, Window};

const BOARD_SIZE: usize = 100;
const CELL_SIZE: usize = 10;

fn main()
{
    simple_logger::SimpleLogger::new().init().unwrap();

    let window =
        Window::new_centered("Speedy2D: Mouse Grab Example", (900, 900)).unwrap();

    let board:[[bool; BOARD_SIZE];BOARD_SIZE] = [[false;BOARD_SIZE];BOARD_SIZE];

    window.run_loop(MyWindowHandler {
        hovering: Vec2::ZERO,
        window_size: UVec2::ZERO,
        board
    })
}

fn update_board(tmpboard : [[bool; BOARD_SIZE]; BOARD_SIZE]) -> [[bool; BOARD_SIZE]; BOARD_SIZE] {
    let mut board: [[bool; BOARD_SIZE]; BOARD_SIZE] = [[false; BOARD_SIZE]; BOARD_SIZE];
    for i in 0..board.len() {
        for j in 0..board.len() {
            let mut count = 0;
            if i != 0 && j != 0 && tmpboard[i - 1][j - 1] {
                count += 1;
            }
            if i != 0 && tmpboard[i - 1][j]{ 
                count += 1;
            }
            if j != board.len() - 1 && i != 0 && tmpboard[i - 1][j + 1] {
                count += 1;
            }
            if j > 0 && tmpboard[i][j - 1] {
                count += 1;
            }
            if j < board.len() - 1 && tmpboard[i][j + 1] {
                count += 1;
            }
            if i != board.len() - 1 && j != 0 && tmpboard[i + 1][j - 1] {
                count += 1;
            }
            if i != board.len() - 1 && tmpboard[i + 1][j] {
                count += 1;
            }
            if i != board.len() - 1 && j != board.len() - 1 && tmpboard[i + 1][j + 1] {
                count += 1;
            }
            if tmpboard[i][j] {
                if count == 2 || count == 3 {
                    board[i][j] = true;
                } else {
                    board[i][j] = false;
                }
            } else {
                if count == 3 {
                    board[i][j] = true;
                } else {
                    board[i][j] = false;
                }
            }
        }
    }
    return board;
}

// 0 - 10 -> 5
// 11 - 20 -> 15
// 21 - 30 -> 25

fn convert_to_coord(coord: Vec2) -> Vec2 {
    let x = coord.x as i32;
    let y = coord.y as i32;
    // log::info!(
    //     "Got on_mouse_move callback: ({}, {})",
    //     x,
    //     y
    // );
    let x = x / CELL_SIZE as i32;
    let y = y / CELL_SIZE as i32;
    let x = x * CELL_SIZE as i32;
    let y = y * CELL_SIZE as i32;
    return Vec2::new(x as f32, y as f32);
}

struct MyWindowHandler
{
    hovering: Vec2,
    window_size: UVec2,
    board: [[bool; BOARD_SIZE]; BOARD_SIZE]
}

impl WindowHandler for MyWindowHandler
{
    fn on_start(&mut self, _helper: &mut WindowHelper, info: WindowStartupInfo)
    {
        // log::info!("Got on_start callback: {:?}", info);
        self.window_size = *info.viewport_size_pixels();
    }

    fn on_resize(&mut self, _helper: &mut WindowHelper<()>, size_pixels: UVec2)
    {
        self.window_size = size_pixels;
    }

    fn on_draw(&mut self, _helper: &mut WindowHelper, graphics: &mut Graphics2D)
    {
        // Clear the screen
        graphics.clear_screen(Color::from_rgb(0.0, 0.0, 0.0));

        // Red for down, blue for up
        let colorgrey = Color::from_rgb(0.6, 0.8, 1.0);
        let colorwhite = Color::from_rgb(1.0, 1.0, 1.0);

        // Draw a circle at the mouse pointer location
        graphics.draw_circle(convert_to_coord(self.hovering), 5.0, colorgrey);
        
        for i in 0..self.board.len() {
            for j in 0..self.board.len() { 
                if self.board[i][j] {
                    graphics.draw_circle(Vec2::new(i as f32 * CELL_SIZE as f32, j as f32 * CELL_SIZE as f32), 5.0, colorwhite);
                }
            }
        }
    }

    fn on_mouse_move(&mut self, helper: &mut WindowHelper, position: Vec2)
    {
        // log::info!(
        //     "Got on_mouse_move callback: ({:.1}, {:.1})",
        //     position.x,
        //     position.y,
        // );

        self.hovering.x = position.x;
        self.hovering.y = position.y;

        helper.request_redraw();
    }

    fn on_mouse_button_down(&mut self, helper: &mut WindowHelper, button: MouseButton)
    {
        let x = self.hovering.x as usize;
        let y = self.hovering.y as usize;
        // log::info!(
        //     "Got on_mouse_move callback: ({:.1}, {:.1}, {}, {}, {:?})",
        //     self.hovering.x,
        //     self.hovering.y,
        //     x/CELL_SIZE,
        //     y/CELL_SIZE,
        //     button
        // );

        if button == MouseButton::Left {
            if (x/CELL_SIZE) < BOARD_SIZE && (y/CELL_SIZE) < BOARD_SIZE {
                self.board[x/CELL_SIZE][y/CELL_SIZE] = !self.board[x/CELL_SIZE][y/CELL_SIZE];
            }
        } else {
            self.board = update_board(self.board);
        }
        helper.request_redraw();
    }
}