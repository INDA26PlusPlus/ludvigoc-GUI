use ggez::event;
use ggez::graphics::{self, Color, Rect};
use ggez::{Context, GameResult};
use ggez::glam::*;

struct MainState {
}

impl MainState {
    fn new() -> GameResult<MainState> {
        let s = MainState {  };
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
            Rect::new(0.0, 0.0, 70.0, 70.0),
            Color::WHITE,
        )?;
        let dark_square = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            Rect::new(0.0, 0.0, 70.0, 70.0),
            Color::BLACK,
        )?;
        for rad in 0..8{
            for kolumn in 0..8{
                let square = if (rad + kolumn) % 2 == 0 {
                    &light_square
                } else {
                    &dark_square
                };
                let x = kolumn as f32 * 70.0;
                let y = rad as f32 * 70.0;
                canvas.draw(square, Vec2::new(x, y));
            }
        }

        canvas.finish(ctx)?;
        Ok(())
    }
}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("super_simple", "ggez");
    let (ctx, event_loop) = cb.build()?;
    let state = MainState::new()?;
    event::run(ctx, event_loop, state)
}
