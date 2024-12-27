use ::rand::Rng;
use macroquad::prelude::*;

pub const SCREEN_WIDTH: f32 = 800.0;
pub const SCREEN_HEIGHT: f32 = 600.0;
pub const BAR_HEIGHT: f32 = 20.0;
pub const INITIAL_BAR_WIDTH: f32 = 150.0;
pub const BALL_SIZE: f32 = 20.0;

pub struct Game {
    pub ball_x: f32,
    pub ball_y: f32,
    pub ball_dx: f32,
    pub ball_dy: f32,
    pub bar_x: f32,
    pub bar_width: f32,
    pub score: usize,
    pub high_score: usize,
    pub hearts: usize,
    pub milestones: Vec<usize>,
    pub game_over: bool,
    pub text_timer: f32,
    pub text_animation: String,
    pub restart_msg_shown: bool,
}

impl Game {
    pub fn new() -> Self {
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

    pub fn reset_ball(&mut self) {
        let mut rng = ::rand::thread_rng();
        self.ball_x = rng.gen_range(BALL_SIZE..SCREEN_WIDTH - BALL_SIZE);
        self.ball_y = rng.gen_range(BALL_SIZE..SCREEN_HEIGHT / 2.0);
        self.ball_dx = if rng.gen_bool(0.5) { 3.0 } else { -3.0 };
        self.ball_dy = 3.0;
    }

    pub fn move_bar(&mut self) {
        if is_key_down(KeyCode::Left) && self.bar_x > 0.0 {
            self.bar_x -= 5.0;
        }
        if is_key_down(KeyCode::Right) && self.bar_x + self.bar_width < SCREEN_WIDTH {
            self.bar_x += 5.0;
        }
    }

    pub fn award_heart(&mut self) {
        self.hearts += 1;
    }

    pub fn lose_heart(&mut self) {
        if self.hearts > 0 {
            self.hearts -= 1;
        }
    }

    pub fn set_game_over(&mut self) {
        self.high_score = self.high_score.max(self.score);
        self.game_over = true;
        self.text_animation = "GAME OVER!".to_string();
        self.text_timer = 3.0;
    }

    pub fn update(&mut self) {
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

        self.ball_x += self.ball_dx;
        self.ball_y += self.ball_dy;

        if self.ball_x <= 0.0 || self.ball_x + BALL_SIZE >= SCREEN_WIDTH {
            self.ball_dx *= -1.0;
        }
        if self.ball_y <= 0.0 {
            self.ball_dy *= -1.0;
        }

        if self.ball_y + BALL_SIZE >= SCREEN_HEIGHT - BAR_HEIGHT
            && self.ball_x + BALL_SIZE >= self.bar_x
            && self.ball_x <= self.bar_x + self.bar_width
        {
            self.ball_dy *= -1.0;
            self.score += 1;

            if let Some(pos) = self
                .milestones
                .iter()
                .position(|&milestone| milestone == self.score)
            {
                self.award_heart();
                self.bar_width = (self.bar_width + 15.0).min(300.0);
                self.milestones.remove(pos);
            }
        }

        if self.ball_y + BALL_SIZE >= SCREEN_HEIGHT {
            self.lose_heart();
            self.bar_width = (self.bar_width - 20.0).max(50.0);
            self.reset_ball();

            if self.hearts == 0 {
                self.set_game_over();
            }
        }
    }

    pub fn restart(&mut self) {
        self.score = 0;
        self.hearts = 3;
        self.bar_width = INITIAL_BAR_WIDTH;
        self.milestones = vec![5, 10, 15, 20];
        self.game_over = false;
        self.restart_msg_shown = false;
        self.text_timer = 0.0;
        self.reset_ball();
    }

    pub fn draw(&self) {
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
