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

            // Award hearts for specific milestones
            if let Some(pos) = self.milestones.iter().position(|&m| self.score == m) {
                self.hearts += 1;
                self.bar_width = (self.bar_width + 15.0).min(300.0); // Increase bar width, cap at 300
                self.milestones.remove(pos); // Remove milestone after granting heart
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
            //    self.score.saturating_sub(self.high_score) + 5,
            self.high_score + 5,
            self.high_score + 10,
            self.high_score + 15,
            self.high_score + 20,
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
    fn test_score_increase_on_bar_collision() {
        let mut game = GameState::new();
        let initial_score = game.score;

        // Simulate a ball hitting the bar
        game.ball_y = SCREEN_HEIGHT - BAR_HEIGHT - BALL_SIZE - 1.0;
        game.update(); // This should increase the score

        assert_eq!(game.score, initial_score + 1);
    }
}
