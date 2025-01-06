use bounce_shield::{Game, HEIGHT, WIDTH};
use ggez::conf::{Conf, WindowMode};
use ggez::event;
use ggez::{ContextBuilder, GameResult};

fn main() -> GameResult {
    let (mut ctx, event_loop) = ContextBuilder::new("bounce_shield", "Your Name")
        .default_conf(Conf::new().window_mode(WindowMode::default().dimensions(WIDTH, HEIGHT)))
        .build()?;
    let game = Game::new(&mut ctx)?;
    event::run(ctx, event_loop, game)
}
