use bounce_shield::Game;
use macroquad::prelude::*;

#[macroquad::main("bounce_shield")]
async fn main() {
    let mut game = Game::new();

    set_fullscreen(true);

    loop {
        game.move_bar();
        game.update();
        game.draw();

        if game.game_over && is_key_pressed(KeyCode::R) {
            game.restart();
        }

        next_frame().await;
    }
}
