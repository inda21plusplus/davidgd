extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL, Texture};

use crate::pieces::generate_array_of_all_pieces;
use crate::pieces::is_black;

mod colors;

mod board_logic;
mod pieces;
mod fen;

struct Game {
    gl: GlGraphics,
    board: Board
}

impl Game {
    fn render(&mut self, arg: &RenderArgs) {

        self.gl.draw(arg.viewport(), |_c, gl| {
            graphics::clear(colors::black(), gl);
        });

        self.board.render_alternating_colored_tiles(&mut self.gl, arg);
    }
}

struct Board {
    board_array: [[u8; 8]; 8],
    light_color: [f32; 4],
    dark_color: [f32; 4],
    size_in_pixels: u32
}

impl Board {

    fn render_alternating_colored_tiles(&mut self, gl: &mut GlGraphics, args: &RenderArgs) {

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;

            for (row, x_tiles) in self.board_array.iter_mut().enumerate() {
                for (col, y_tiles) in x_tiles.iter_mut().enumerate() {

                    let tile = graphics::rectangle::square(
                        (row * (self.size_in_pixels as usize) / 8) as f64,
                        (col * (self.size_in_pixels as usize) / 8) as f64,
                    ((self.size_in_pixels as usize) / 8) as f64);
                    
                    let mut tile_color = self.dark_color;

                    if (row + col) % 2 == 0 {
                        tile_color = self.light_color;
                    }

                    graphics::rectangle(tile_color, tile, transform, gl);
                }
            }
        });
    }
}

fn main() {
    // let opengl = OpenGL::V4_3;

    // let window_size = 900;

    // let mut window: GlutinWindow = WindowSettings::new(
    //     "Chess", 
    //     [window_size, window_size],
    // ).opengl(opengl)
    // .exit_on_esc(true)
    // .build()
    // .unwrap();

    let mut chess_board = board_logic::generate_8_x_8_board_array();

    // let mut game = Game {
    //     gl: GlGraphics::new(opengl),
    //     board: Board { board_array: chess_board, light_color: colors::white(), dark_color: colors::black(), size_in_pixels: window_size },
    // };

    // let pieces = generate_array_of_all_pieces();
    
    // println!("{:?}", pieces);
    // println!("{:?}", pieces::get_black_king());
    // println!("{:?}", pieces::get_black_king());
    // println!("{:?}", is_black(pieces::get_black_queen()));
    fen::set_starting_position(chess_board);

    // println!("{:?}", pieces::pawn());
    // println!("{:?}", pieces::rook());
    // println!("{:?}", pieces::black());
    // println!("{:?}", pieces::white());

    // let mut pawn = &format!("0{:b} ", pieces::pawn());
    // let mut pawn = &format!("0{:b} ", pieces::white());

    // println!("{:?}", pieces::white() & pieces::pawn());

    // let mut events = Events::new(EventSettings::new());
    // while let Some(e) = events.next(&mut window) {

    //     if let Some(r) = e.render_args() {
    //         game.render(&r);
    //     }
    // }
}