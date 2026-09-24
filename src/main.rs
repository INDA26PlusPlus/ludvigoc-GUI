use ggez::event;
use ggez::graphics::{self, Color, Rect};
use ggez::{Context, GameResult};
use ggez::glam::*;
use chess_library::*;

const SQUARE_SIZE: f32 = 70.0;

struct MainState {
    board: Board,
}

impl MainState {
    fn new(board: Board) -> GameResult<MainState> {
        let s = MainState { board };
        Ok(s)
    }
}

impl event::EventHandler for MainState{
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(
            ctx,
            graphics::Color::from([0.1, 0.2, 0.3, 1.0]),
        );
        let light_square = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            Rect::new(0.0, 0.0, SQUARE_SIZE, SQUARE_SIZE),
            Color::WHITE,
        )?;
        let dark_square = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            Rect::new(0.0, 0.0, SQUARE_SIZE, SQUARE_SIZE),
            Color::BLACK,
        )?;
        for rad in 0..8{
            for kolumn in 0..8{
                let square = if (rad + kolumn) % 2 == 0 {
                    &light_square
                } else {
                    &dark_square
                };
                let x = kolumn as f32 * SQUARE_SIZE;
                let y = rad as f32 * SQUARE_SIZE;
                canvas.draw(square, Vec2::new(x, y));
            }
        }

        for ruta in 0..64{
            let piece = Board::piece_type_on_position(&self.board, ruta);
            if piece != -1 {
                let symbol = match piece {
                    0 => "wP",
                    1 => "wR",
                    2 => "wN",
                    3 => "wB",
                    4 => "wK",
                    5 => "wQ",
                    6 => "bP",
                    7 => "bR",
                    8 => "bN",
                    9 => "bB",
                    10 => "bK",
                    11 => "bQ",
                    _ => "",
                };


                let text = graphics::Text::new(symbol);

                let kolumn = ruta % 8;
                let rad = ruta / 8;

                let x = (7.0f32 - kolumn as f32) * SQUARE_SIZE;
                let y = (7.0f32 - rad as f32) * SQUARE_SIZE;

                canvas.draw(
                    &text,
                    graphics::DrawParam::default()
                        .dest(Vec2::new(x + 25.0, y + 20.0))
                        .color(Color::RED),
                );
            }
        }

        canvas.finish(ctx)?;
        Ok(())
    }
}

pub fn main() -> GameResult {

    let initial_boards: [u64; 12] = [0; 12];

    let mut board = Board {
        boards: initial_boards,
        w_enpesant: 0,
        b_enpesant: 0,

        w_l_rook_moved: false,
        w_r_rook_moved: false,
        w_k_moved: false,

        b_l_rook_moved: false,
        b_r_rook_moved: false,
        b_k_moved: false,

        white_turn: true,
    };

    Board::set_standard_board(&mut board);
    
    let cb = ggez::ContextBuilder::new("super_simple", "ggez");
    let (ctx, event_loop) = cb.build()?;
    let state = MainState::new(board)?;
    event::run(ctx, event_loop, state)
}
