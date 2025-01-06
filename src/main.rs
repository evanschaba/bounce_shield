use bounce_shield::{Game, create_game_ctx};
use ggez::event;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut ctx, event_loop) = create_game_ctx()?;
    let game = Game::new(&mut ctx)?;
    event::run(ctx, event_loop, game)
}
