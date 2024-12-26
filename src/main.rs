use ::rand::Rng;
use macroquad::prelude::*;

const SCREEN_WIDTH: f32 = 800.0;
const SCREEN_HEIGHT: f32 = 600.0;
const BAR_HEIGHT: f32 = 20.0;
const INITIAL_BAR_WIDTH: f32 = 150.0;
const BALL_SIZE: f32 = 20.0;

struct GameState {
    ball_x: f32,
    ball_y: f32,
    ball_dx: f32,
    ball_dy: f32,
    bar_x: f32,
    bar_width: f32,
    score: usize,
    high_score: usize,
    hearts: usize,
    milestones: Vec<usize>, // Score milestones for awarding hearts
    game_over: bool,
    text_timer: f32,
    text_animation: String,
    restart_msg_shown: bool, // Flag to display "Press R to retry" message
}

impl GameState {
    fn new() -> Self {
        let mut rng = ::rand::thread_rng();
        Self {
            ball_x: rng.gen_range(BALL_SIZE..SCREEN_WIDTH - BALL_SIZE),
            ball_y: rng.gen_range(BALL_SIZE..SCREEN_HEIGHT / 2.0),
            ball_dx: if rng.gen_bool(0.5) { 3.0 } else { -3.0 },
            ball_dy: 3.0,
            bar_x: (SCREEN_WIDTH - INITIAL_BAR_WIDTH) / 2.0,
            bar_width: INITIAL_BAR_WIDTH,
            score: 0,
            high_score: 0,
            hearts: 3,
            milestones: vec![], // No milestones initially
            game_over: false,
            text_timer: 0.0,
            text_animation: "GAME START!".to_string(),
            restart_msg_shown: false,
        }
    }

    fn reset_ball(&mut self) {
        let mut rng = ::rand::thread_rng();
        self.ball_x = rng.gen_range(BALL_SIZE..SCREEN_WIDTH - BALL_SIZE);
        self.ball_y = rng.gen_range(BALL_SIZE..SCREEN_HEIGHT / 2.0);
        self.ball_dx = if rng.gen_bool(0.5) { 3.0 } else { -3.0 };
        self.ball_dy = 3.0;
    }

    fn move_bar(&mut self) {
        if is_key_down(KeyCode::Left) && self.bar_x > 0.0 {
            self.bar_x -= 5.0;
        }
        if is_key_down(KeyCode::Right) && self.bar_x + self.bar_width < SCREEN_WIDTH {
            self.bar_x += 5.0;
        }
    }
    fn update(&mut self) {
        if self.text_timer > 0.0 {
            self.text_timer -= get_frame_time();
            return;
        }

        if self.game_over {
            if !self.restart_msg_shown {
                self.text_animation = "Press R to retry!".to_string();
                self.text_timer = 3.0;
                self.restart_msg_shown = true;
            }
            return;
        }

        // Ball movement
        self.ball_x += self.ball_dx;
        self.ball_y += self.ball_dy;

        // Ball collision with walls
        if self.ball_x <= 0.0 || self.ball_x + BALL_SIZE >= SCREEN_WIDTH {
            self.ball_dx *= -1.0;
        }
        if self.ball_y <= 0.0 {
            self.ball_dy *= -1.0;
        }

        // Ball collision with bar
        if self.ball_y + BALL_SIZE >= SCREEN_HEIGHT - BAR_HEIGHT
            && self.ball_x + BALL_SIZE >= self.bar_x
            && self.ball_x <= self.bar_x + self.bar_width
        {
            self.ball_dy *= -1.0;
            self.ball_y = SCREEN_HEIGHT - BAR_HEIGHT - BALL_SIZE - 1.0; // Move ball above the bar
            self.score += 1;

            // Award hearts and increase bar width for specific milestones
            let mut crossed_milestones = Vec::new();
            for (i, &milestone) in self.milestones.iter().enumerate() {
                if self.score >= milestone {
                    self.hearts += 1;
                    self.bar_width = (self.bar_width + 15.0).min(300.0); // Increase bar width, cap at 300
                    crossed_milestones.push(i); // Track crossed milestones
                }
            }

            // Remove crossed milestones
            for &index in crossed_milestones.iter().rev() {
                self.milestones.remove(index);
            }
        }

        // Ball falls below the bar
        if self.ball_y + BALL_SIZE >= SCREEN_HEIGHT {
            self.hearts -= 1;
            self.bar_width = (self.bar_width - 20.0).max(50.0); // Decrease bar width, minimum of 50
            self.reset_ball();

            if self.hearts == 0 {
                self.high_score = self.high_score.max(self.score); // Update high score here
                self.game_over = true;
                self.text_animation = "GAME OVER!".to_string();
                self.text_timer = 3.0;
                self.restart_msg_shown = false; // Reset the message flag
            }
        }
    }
    fn restart(&mut self) {
        self.score = 0;
        self.hearts = 3;
        self.bar_width = INITIAL_BAR_WIDTH; // Reset bar width
        self.milestones = vec![
            self.high_score + 5,
            self.high_score + 10,
            self.high_score + 15,
            self.high_score + 20, // Added +20 milestone to the test
        ]; // Reset milestones
        self.game_over = false;
        self.reset_ball();
    }

    fn draw(&self) {
        clear_background(BLACK);

        // Draw ball
        draw_circle(
            self.ball_x + BALL_SIZE / 2.0,
            self.ball_y + BALL_SIZE / 2.0,
            BALL_SIZE / 2.0,
            WHITE,
        );

        // Draw bar
        draw_rectangle(
            self.bar_x,
            SCREEN_HEIGHT - BAR_HEIGHT,
            self.bar_width,
            BAR_HEIGHT,
            WHITE,
        );

        // Draw score, high score, and hearts
        draw_text(&format!("Score: {}", self.score), 20.0, 30.0, 30.0, GREEN);
        draw_text(
            &format!("High Score: {}", self.high_score),
            20.0,
            60.0,
            30.0,
            YELLOW,
        );
        draw_text(&format!("Hearts: {}", self.hearts), 20.0, 90.0, 30.0, RED);

        // Draw animated text
        if self.text_timer > 0.0 {
            let scale = 1.0 + (3.0 - self.text_timer) * 0.5;
            draw_text_ex(
                &self.text_animation,
                SCREEN_WIDTH / 2.0 - 150.0 * scale,
                SCREEN_HEIGHT / 2.0,
                TextParams {
                    font_size: (40.0 * scale) as u16,
                    color: WHITE,
                    ..Default::default()
                },
            );
        }
    }
}

#[macroquad::main("BounceShield")]
async fn main() {
    let mut game = GameState::new();

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ball_reset_position() {
        let mut game = GameState::new();
        let initial_x = game.ball_x;
        let initial_y = game.ball_y;

        game.reset_ball();

        // Ensure the ball position is different after reset
        assert_ne!(game.ball_x, initial_x);
        assert_ne!(game.ball_y, initial_y);
    }

    #[test]
    fn test_ball_collision_with_walls() {
        let mut game = GameState::new();
        // Move ball to the left edge
        game.ball_x = 0.0;
        game.ball_dx = -3.0;
        game.update();
        // Ensure ball bounces back after hitting the wall
        assert!(game.ball_dx > 0.0);

        // Move ball to the top edge
        game.ball_y = 0.0;
        game.ball_dy = -3.0;
        game.update();
        // Ensure ball bounces back after hitting the top wall
        assert!(game.ball_dy > 0.0);

        // Move ball to the right edge
        game.ball_x = SCREEN_WIDTH - BALL_SIZE;
        game.ball_dx = 3.0;
        game.update();
        // Ensure ball bounces back after hitting the right wall
        assert!(game.ball_dx < 0.0);
    }

    #[test]
    fn test_ball_collision_with_bar() {
        let mut game = GameState::new();
        game.ball_y = SCREEN_HEIGHT - BAR_HEIGHT - BALL_SIZE - 1.0; // Ball just above the bar
        game.ball_x = game.bar_x + game.bar_width / 2.0 - BALL_SIZE / 2.0; // Ball in the middle of the bar
        game.update();

        // Ensure the ball bounces back after colliding with the bar
        assert!(game.ball_dy < 0.0);
        assert_eq!(game.score, 1); // Score should increase by 1
    }

    #[test]
    fn test_heart_loss_and_game_over() {
        let mut game = GameState::new();
        game.hearts = 1; // Set hearts to 1 to trigger game over on the next fall
        game.ball_y = SCREEN_HEIGHT - BAR_HEIGHT + 1.0; // Ball falls below the bar
        game.update();

        // Ensure hearts are deducted and game over is triggered
        assert_eq!(game.hearts, 0);
        assert!(game.game_over);
        assert_eq!(game.text_animation, "GAME OVER!");
    }

    #[test]
    fn test_restart_resets_game_state() {
        let mut game = GameState::new();
        game.score = 10;
        game.hearts = 0;
        game.bar_width = 120.0;
        game.restart();

        // Ensure game state is reset correctly
        assert_eq!(game.score, 0);
        assert_eq!(game.hearts, 3);
        assert_eq!(game.bar_width, INITIAL_BAR_WIDTH);
    }
    // [TODO] Add more tests for milestone hearts award and bar width increase
    // IMPROVE THE IMPLEMENTATION ABIT MORE THERE'S A MISMATCH IN THE update() FUNCTION
    // #[test]
    // fn test_milestone_hearts_award() {
    //     let mut game = GameState::new();

    //     // Set the score to the first milestone
    //     game.score = game.high_score + 5;

    //     // Set the milestones correctly (e.g., `high_score + 5`, `high_score + 10`)
    //     game.milestones = vec![
    //         game.high_score + 5,
    //         game.high_score + 10,
    //         game.high_score + 15,
    //     ];

    //     // Call update to process the score and milestones
    //     game.update();

    //     // Ensure hearts are awarded after reaching milestones
    //     assert_eq!(game.hearts, 4); // Awarded one heart
    // }

    // #[test]
    // fn test_bar_is_increased_upon_milestone() {
    //     let mut game = GameState::new();

    //     // Set the score to the first milestone
    //     game.score = game.high_score + 5;

    //     // Set the milestones correctly (e.g., `high_score + 5`, `high_score + 10`)
    //     game.milestones = vec![
    //         game.high_score + 5,
    //         game.high_score + 10,
    //         game.high_score + 15,
    //     ];

    //     // Call update to process the score and milestones
    //     game.update();

    //     // Ensure bar width increased (or capped at 300)
    //     assert_eq!(game.bar_width, 165.0); // Bar width increased by 15
}
