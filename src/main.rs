mod network;

use chess_library::*;
use ggez::event;
use ggez::glam::*;
use ggez::graphics::{self, Color, Rect};
use ggez::{Context, GameResult};
use std::io::{Read, Write};
use std::net::TcpStream;

const SQUARE_SIZE: f32 = 70.0;

struct MainState {
    board: Board,
    selected_square: Option<usize>,
    winner: Option<&'static str>,
    color: char,
    stream: TcpStream,
}

impl MainState {
    fn new(board: Board, stream: TcpStream, color: char) -> GameResult<MainState> {
        let s = MainState {
            board,
            selected_square: None,
            winner: None,
            color,
            stream,
        };
        Ok(s)
    }

    fn send_move(&mut self, from: usize, to: usize) {
        let from_protocol = 63 - from;
        let to_protocol = 63 - to;

        let from_file = (b'A' + (from_protocol % 8) as u8) as char;
        let from_rank = (b'1' + (7 - from_protocol / 8) as u8) as char;

        let to_file = (b'A' + (to_protocol % 8) as u8) as char;
        let to_rank = (b'1' + (7 - to_protocol / 8) as u8) as char;

        let mut message = format!("{}{}{}{}-", from_file, from_rank, to_file, to_rank);

        for protocol_index in 0..64 {
            let board_index = 63 - protocol_index;

            let piece = Board::piece_type_on_position(&self.board, board_index);

            let symbol = match piece {
                -1 => '-',
                0 => 'P',
                1 => 'R',
                2 => 'N',
                3 => 'B',
                4 => 'K',
                5 => 'Q',
                6 => 'p',
                7 => 'r',
                8 => 'n',
                9 => 'b',
                10 => 'k',
                11 => 'q',
                _ => '-',
            };

            message.push(symbol);
        }

        message.push('\n');

        self.stream
            .write_all(message.as_bytes())
            .expect("Failed to send move");
    }

    fn receive_message(&mut self) -> String {
        let mut message = String::new();
        let mut buffer = [0; 1];

        loop {
            self.stream
                .read_exact(&mut buffer)
                .expect("Failed to receive message");

            if buffer[0] == b'\n' {
                break;
            }

            message.push(buffer[0] as char);
        }

        message
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        if (self.board.white_turn && self.color == 'B')
            || (!self.board.white_turn && self.color == 'W')
        {
            let message = self.receive_message();

            if message == "OK" {
                return Ok(());
            }

            if message == "REJECT" {
                return Ok(());
            }

            // opponents move
            let chars: Vec<char> = message.chars().collect();

            let from_file = chars[0];
            let from_rank = chars[1];

            let to_file = chars[2];
            let to_rank = chars[3];

            let from_protocol =
                ('8' as usize - from_rank as usize) * 8 + (from_file as usize - 'A' as usize);

            let to_protocol =
                ('8' as usize - to_rank as usize) * 8 + (to_file as usize - 'A' as usize);

            let from = 63 - from_protocol;
            let to = 63 - to_protocol;

            self.board.move_piece(from, to as u64, None);

            self.stream
                .write_all(b"OK\n")
                .expect("Failed to send response");
        }

        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        button: ggez::input::mouse::MouseButton,
        x: f32,
        y: f32,
    ) -> GameResult {
        if self.board.white_turn && self.color == 'B' || !self.board.white_turn && self.color == 'W'
        {
            return Ok(());
        }

        if button == ggez::input::mouse::MouseButton::Left {
            let kolumn = 7 - (x / SQUARE_SIZE) as usize;
            let rad = 7 - (y / SQUARE_SIZE) as usize;
            let ruta = rad * 8 + kolumn;

            match self.selected_square {
                None => {
                    self.selected_square = Some(ruta);
                }
                Some(from) => {
                    self.board.move_piece(from as usize, ruta as u64, None);
                    self.send_move(from, ruta);
                    self.selected_square = None;

                    if Board::is_mate_white(&self.board) {
                        self.winner = Some("Svart vann!");
                    }

                    if Board::is_mate_black(&self.board) {
                        self.winner = Some("Vit vann!");
                    }
                }
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));
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
        for rad in 0..8 {
            for kolumn in 0..8 {
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

        for ruta in 0..64 {
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

        if let Some(winner) = self.winner {
            let text = graphics::Text::new(winner);

            canvas.draw(
                &text,
                graphics::DrawParam::default()
                    .dest(Vec2::new(250.0, 250.0))
                    .color(Color::RED),
            );
        }

        canvas.finish(ctx)?;
        Ok(())
    }
}

pub fn main() -> GameResult {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 3 {
        println!("use --server or --client and chose color (W or B)");
        return Ok(());
    }

    let color = args[2].chars().nth(0).unwrap_or('W');

    let stream = if args[1] == "--server" {
        network::start_server(color)
    } else if args[1] == "--client" {
        network::connect_to_server(color)
    } else {
        println!("Använd --server eller --client");
        return Ok(());
    };

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
    let state = MainState::new(board, stream, color)?;
    event::run(ctx, event_loop, state)
}
