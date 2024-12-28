use bounce_shield::Game;
use bounce_shield::{SCREEN_HEIGHT, SCREEN_WIDTH};
use macroquad::prelude::*;

/// Logic is now broken again, after game retry, the fullscreen issue is fixed for now
///
/// next: handle the game logic, issues
///
/// 0. after dropping the ball 3 times, press 'r' to retry takes forever to animate in
/// 1. after retry, if you drop the ball once, the game over text is displayed then a retry option is offered
/// 2. milestones are somehow reached even with a highscore of 0?? bruh.. everyone gets a trophy 4 participating??
#[macroquad::main("Bounce Shield")]
async fn main() {
    let mut game = Game::new();

    set_fullscreen(true);

    loop {
        // Dynamically fetch window dimensions
        let window_width = screen_width();
        let window_height = screen_height();

        // Calculate aspect ratio scaling
        let scale = f32::min(window_width / SCREEN_WIDTH, window_height / SCREEN_HEIGHT);
        let scaled_width = SCREEN_WIDTH * scale;
        let scaled_height = SCREEN_HEIGHT * scale;

        // Center the game in the window
        let offset_x = (window_width - scaled_width) / 2.0;
        let offset_y = (window_height - scaled_height) / 2.0;

        // Set up the camera with fixed virtual resolution
        set_camera(&Camera2D {
            zoom: vec2(2.0 / SCREEN_WIDTH, 2.0 / SCREEN_HEIGHT),
            target: vec2(SCREEN_WIDTH / 2.0, SCREEN_HEIGHT / 2.0),
            viewport: Some((
                offset_x as i32,
                offset_y as i32,
                scaled_width as i32,
                scaled_height as i32,
            )), // Corrected to tuple
            render_target: None,
            rotation: 0.0,
            offset: vec2(0.0, 0.0), // Added the missing `offset` field
        });

        game.move_bar();
        game.update();
        game.draw();

        // Restart if game over and 'R' is pressed
        if game.game_over && is_key_pressed(KeyCode::R) {
            game.restart();
        }

        next_frame().await;
    }
}
