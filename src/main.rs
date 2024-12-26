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
    milestones: Vec<usize>,
    game_over: bool,
    text_timer: f32,
    text_animation: String,
    restart_msg_shown: bool,
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
            milestones: vec![5, 10, 15, 20],
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

    fn award_heart(&mut self) {
        self.hearts += 1;
    }

    fn lose_heart(&mut self) {
        if self.hearts > 0 {
            self.hearts -= 1;
        }
    }

    fn set_game_over(&mut self) {
        self.high_score = self.high_score.max(self.score);
        self.game_over = true;
        self.text_animation = "GAME OVER!".to_string();
        self.text_timer = 3.0;
    }
    fn update(&mut self) {
        if self.text_timer > 0.0 {
            self.text_timer -= get_frame_time();
            return;
        }

        if self.game_over {
            if !self.restart_msg_shown {
                self.text_animation = "Press R to retry!".to_string();
                self.restart_msg_shown = true;
            }
            return;
        }

        // Update ball position
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
            self.ball_dy *= -1.0; // Reverse ball direction
            self.score += 1; // Increment score

            // Check for milestone rewards
            if let Some(pos) = self
                .milestones
                .iter()
                .position(|&milestone| milestone == self.score)
            {
                self.award_heart(); // Award a heart
                self.bar_width = (self.bar_width + 15.0).min(300.0); // Increase bar width
                self.milestones.remove(pos); // Remove the milestone
            }
        }

        // Ball falls below the bar
        if self.ball_y + BALL_SIZE >= SCREEN_HEIGHT {
            self.lose_heart(); // Deduct a heart
            self.bar_width = (self.bar_width - 20.0).max(50.0); // Shrink bar width
            self.reset_ball(); // Reset the ball position

            // Check for game over
            if self.hearts == 0 {
                self.set_game_over();
            }
        }
    }

    fn restart(&mut self) {
        self.score = 0;
        self.hearts = 3;
        self.bar_width = INITIAL_BAR_WIDTH;
        self.milestones = vec![5, 10, 15, 20];
        self.game_over = false;
        self.restart_msg_shown = false;
        self.text_timer = 0.0;
        self.reset_ball();
    }

    fn draw(&self) {
        clear_background(BLACK);

        draw_circle(
            self.ball_x + BALL_SIZE / 2.0,
            self.ball_y + BALL_SIZE / 2.0,
            BALL_SIZE / 2.0,
            WHITE,
        );

        draw_rectangle(
            self.bar_x,
            SCREEN_HEIGHT - BAR_HEIGHT,
            self.bar_width,
            BAR_HEIGHT,
            WHITE,
        );

        draw_text(&format!("Score: {}", self.score), 20.0, 30.0, 30.0, GREEN);
        draw_text(
            &format!("High Score: {}", self.high_score),
            20.0,
            60.0,
            30.0,
            YELLOW,
        );
        draw_text(&format!("Hearts: {}", self.hearts), 20.0, 90.0, 30.0, RED);

        if self.text_timer > 0.0 {
            draw_text_ex(
                &self.text_animation,
                SCREEN_WIDTH / 2.0 - 100.0,
                SCREEN_HEIGHT / 2.0,
                TextParams {
                    font_size: 40,
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

    #[test]
    fn test_milestone_hearts_award() {
        let mut game = GameState::new();

        // Simulate a scenario where the ball is about to hit the bar and score a milestone
        game.score = 4; // Just before the milestone
        game.milestones = vec![5]; // Set the next milestone
        game.ball_y = SCREEN_HEIGHT - BAR_HEIGHT - BALL_SIZE - 1.0; // Ball just above the bar
        game.ball_x = game.bar_x + game.bar_width / 2.0 - BALL_SIZE / 2.0; // Ball aligned to hit the middle of the bar

        game.update();

        // Check milestone logic
        assert_eq!(game.score, 5); // Score should now match the milestone
        assert_eq!(game.hearts, 4); // Heart should be awarded
        assert!(game.bar_width > INITIAL_BAR_WIDTH); // Bar width should increase
        assert!(!game.milestones.contains(&5)); // Milestone should be removed
    }

    #[test]
    fn test_bar_width_limits() {
        let mut game = GameState::new();
        game.bar_width = 300.0; // Max width
        game.score = 4; // One less than milestone
        game.milestones = vec![5]; // Next milestone at 5

        game.ball_y = SCREEN_HEIGHT - BAR_HEIGHT - BALL_SIZE - 1.0; // Ball hits the bar
        game.ball_x = game.bar_x + game.bar_width / 2.0;

        game.update();

        // Ensure width is capped
        assert_eq!(game.bar_width, 300.0);
    }
}
