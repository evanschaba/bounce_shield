use ggez::graphics::{Drawable, Text, TextFragment};
use ggez::input::keyboard::{KeyCode, KeyInput};
use ggez::{
    Context, GameResult,
    conf::WindowMode,
    event::EventHandler,
    graphics::{self, Color, DrawParam},
};
use rand::Rng;
use std::time::{Duration, Instant};

pub const WIDTH: f32 = 800.0;
pub const HEIGHT: f32 = 600.0;
pub const BALL_SIZE: f32 = 20.0;
pub const BAR_WIDTH: f32 = 150.0;
pub const BAR_HEIGHT: f32 = 20.0;
pub const BAR_SPEED: f32 = 10.0;
pub const BALL_SPEED: f32 = 5.0;
pub const INITIAL_HEARTS: usize = 3;

pub struct AnimatedText {
    pub text: String,
    pub start_time: Instant,
    pub duration: Duration,
    pub position: [f32; 2],
    pub scale: f32,
    pub color: Color,
}

impl AnimatedText {
    pub fn new(
        text: String,
        position: [f32; 2],
        duration_secs: u64,
        scale: f32,
        color: Color,
    ) -> Self {
        Self {
            text,
            start_time: Instant::now(),
            duration: Duration::from_secs(duration_secs),
            position,
            scale,
            color,
        }
    }

    pub fn is_active(&self) -> bool {
        Instant::now().duration_since(self.start_time) < self.duration
    }
}

#[derive(PartialEq)]
pub enum GameState {
    Countdown,
    Playing,
    Paused,
    GameOver,
}

pub struct Game {
    pub(self) ball: Ball,
    pub(self) bar: Bar,
    pub score: usize,
    pub high_score: usize,
    pub hearts: usize,
    pub state: GameState,
    pub countdown_start: Option<Instant>,
    pub countdown_value: i32,
    pub fullscreen: bool,
    pub animations: Vec<AnimatedText>,
    pub first_start: bool,
    pub prev_high_score: usize,
}

impl Game {
    pub fn new(_ctx: &mut Context) -> GameResult<Self> {
        let mut game = Self {
            ball: Ball::new(),
            bar: Bar::new(),
            score: 0,
            high_score: 0,
            hearts: INITIAL_HEARTS,
            state: GameState::Countdown,
            countdown_start: Some(Instant::now()),
            countdown_value: 3,
            fullscreen: false,
            animations: Vec::new(),
            first_start: true,
            prev_high_score: 0,
        };
        game.add_animation(
            "Get Ready!".to_string(),
            [WIDTH / 2.0, HEIGHT / 2.0],
            2,
            72.0,
            Color::CYAN,
        );
        Ok(game)
    }

    pub fn add_animation(
        &mut self,
        text: String,
        position: [f32; 2],
        duration_secs: u64,
        scale: f32,
        color: Color,
    ) {
        self.animations.push(AnimatedText::new(
            text,
            position,
            duration_secs,
            scale,
            color,
        ));
    }

    pub fn reset(&mut self) {
        self.ball = Ball::new();
        self.bar = Bar::new();
        self.score = 0;
        self.hearts = INITIAL_HEARTS;
        self.state = GameState::Countdown;
        self.countdown_start = Some(Instant::now());
        self.countdown_value = 3;
        self.first_start = false;
        self.add_animation(
            "Get Ready!".to_string(),
            [WIDTH / 2.0, HEIGHT / 2.0],
            2,
            72.0,
            Color::CYAN,
        );
    }

    pub fn check_high_score(&mut self) {
        if self.score > self.high_score {
            if self.high_score > 0 {
                self.add_animation(
                    format!("New High Score: {}!", self.score),
                    [WIDTH / 2.0, HEIGHT / 2.0],
                    2,
                    48.0,
                    Color::CYAN,
                );
            }
            self.high_score = self.score;

            if !self.first_start && self.score > self.prev_high_score + 5 {
                self.hearts += 1;
                self.add_animation(
                    "Extra Heart Awarded!".to_string(),
                    [WIDTH / 2.0, HEIGHT / 2.0 - 50.0],
                    2,
                    36.0,
                    Color::GREEN,
                );
                self.prev_high_score = self.score;
            }
        }
    }
}

impl EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.animations.retain(|anim| anim.is_active());

        match self.state {
            GameState::GameOver => return Ok(()),
            GameState::Paused => return Ok(()),
            GameState::Countdown => {
                if let Some(start_time) = self.countdown_start {
                    if Instant::now().duration_since(start_time) >= Duration::from_secs(1) {
                        self.countdown_value -= 1;
                        self.countdown_start = Some(Instant::now());
                        if self.countdown_value == 0 {
                            self.state = GameState::Playing;
                            self.add_animation(
                                "Game Start!".to_string(),
                                [WIDTH / 2.0, HEIGHT / 2.0],
                                2,
                                72.0,
                                Color::GREEN,
                            );
                            self.add_animation(
                                "Press 'P' or SPACE to pause".to_string(),
                                [WIDTH / 2.0, HEIGHT / 2.0 + 50.0],
                                3,
                                24.0,
                                Color::WHITE,
                            );
                        }
                    }
                }
                return Ok(());
            }
            GameState::Playing => {
                self.ball.update();

                // Ball-wall collision
                if self.ball.x <= 0.0 || self.ball.x + BALL_SIZE >= WIDTH {
                    self.ball.dx *= -1.0;
                }
                if self.ball.y <= 0.0 {
                    self.ball.dy *= -1.0;
                }

                // Ball-bar collision
                if self.ball.y + BALL_SIZE >= self.bar.y
                    && self.ball.x + BALL_SIZE >= self.bar.x
                    && self.ball.x <= self.bar.x + self.bar.width
                {
                    self.ball.dy *= -1.0;
                    self.score += 1;
                    self.check_high_score();
                }

                // Ball falls off screen
                if self.ball.y > HEIGHT {
                    self.hearts -= 1;
                    if self.hearts == 0 {
                        self.state = GameState::GameOver;
                        self.add_animation(
                            "Game Over!".to_string(),
                            [WIDTH / 2.0, HEIGHT / 2.0],
                            999,
                            72.0,
                            Color::RED,
                        );
                        self.add_animation(
                            "Press 'R' to retry".to_string(),
                            [WIDTH / 2.0, HEIGHT / 2.0 + 50.0],
                            999,
                            36.0,
                            Color::WHITE,
                        );
                    } else {
                        self.add_animation(
                            format!("Lost a heart! {} remaining", self.hearts),
                            [WIDTH / 2.0, HEIGHT / 2.0],
                            2,
                            48.0,
                            Color::RED,
                        );
                        self.ball = Ball::new();
                    }
                }

                // Bar movement
                let keys = ctx.keyboard.pressed_keys();
                if keys.contains(&KeyCode::Left) || keys.contains(&KeyCode::A) {
                    self.bar.move_left();
                }
                if keys.contains(&KeyCode::Right) || keys.contains(&KeyCode::D) {
                    self.bar.move_right();
                }
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);

        // Draw score and lives (top left)
        let score_text = Text::new(
            TextFragment::new(format!(
                "Score: {}\nHearts: {}\nHigh Score: {}",
                self.score, self.hearts, self.high_score
            ))
            .scale(24.0),
        );
        score_text.draw(&mut canvas, DrawParam::default().dest([20.0, 20.0]));

        // Draw controls (top right)
        let controls_text = Text::new(
            TextFragment::new("Controls:\nF - Fullscreen\nP/Space - Pause\nR - Retry").scale(20.0),
        );
        controls_text.draw(
            &mut canvas,
            DrawParam::default().dest([WIDTH - 200.0, 20.0]),
        );

        // Draw countdown or game elements
        if self.state == GameState::Countdown {
            let countdown_text = Text::new(
                TextFragment::new(format!("{}", self.countdown_value))
                    .scale(96.0)
                    .color(Color::CYAN),
            );
            let dims = countdown_text.dimensions(ctx).unwrap();
            countdown_text.draw(
                &mut canvas,
                DrawParam::default()
                    .dest([WIDTH / 2.0 - dims.w / 2.0, HEIGHT / 2.0 - dims.h / 2.0]),
            );
        } else {
            // Draw game objects
            let ball_rect = graphics::Rect::new(self.ball.x, self.ball.y, BALL_SIZE, BALL_SIZE);
            let ball = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                ball_rect,
                Color::WHITE,
            )?;
            ball.draw(&mut canvas, DrawParam::default());

            let bar_rect = graphics::Rect::new(self.bar.x, self.bar.y, self.bar.width, BAR_HEIGHT);
            let bar = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                bar_rect,
                Color::GREEN,
            )?;
            bar.draw(&mut canvas, DrawParam::default());
        }

        // Draw animations
        for anim in &self.animations {
            let text = Text::new(
                TextFragment::new(&anim.text)
                    .scale(anim.scale)
                    .color(anim.color),
            );
            let dims = text.dimensions(ctx).unwrap();
            text.draw(
                &mut canvas,
                DrawParam::default().dest([
                    anim.position[0] - dims.w / 2.0,
                    anim.position[1] - dims.h / 2.0,
                ]),
            );
        }

        canvas.finish(ctx)?;
        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        key_input: KeyInput,
        _repeat: bool,
    ) -> GameResult {
        match key_input.keycode {
            Some(KeyCode::P) | Some(KeyCode::Space) => {
                if self.state == GameState::Playing {
                    self.state = GameState::Paused;
                    self.add_animation(
                        "PAUSED".to_string(),
                        [WIDTH / 2.0, HEIGHT / 2.0],
                        999,
                        72.0,
                        Color::CYAN,
                    );
                } else if self.state == GameState::Paused {
                    self.state = GameState::Playing;
                    self.animations.clear();
                }
            }
            Some(KeyCode::F) => {
                self.fullscreen = !self.fullscreen;
                let mode = if self.fullscreen {
                    WindowMode::default().fullscreen_type(ggez::conf::FullscreenType::True)
                } else {
                    WindowMode::default().fullscreen_type(ggez::conf::FullscreenType::Windowed)
                };
                ctx.gfx.set_mode(mode).expect("Failed to toggle fullscreen");
            }
            Some(KeyCode::R) if self.state == GameState::GameOver => {
                self.prev_high_score = self.high_score;
                self.reset();
            }
            _ => {}
        }
        Ok(())
    }
}

struct Ball {
    x: f32,
    y: f32,
    dx: f32,
    dy: f32,
}

impl Ball {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            x: rng.gen_range(BALL_SIZE..WIDTH - BALL_SIZE),
            y: HEIGHT / 3.0,
            dx: if rng.gen_bool(0.5) {
                BALL_SPEED
            } else {
                -BALL_SPEED
            },
            dy: BALL_SPEED,
        }
    }

    pub fn update(&mut self) {
        self.x += self.dx;
        self.y += self.dy;
    }
}

struct Bar {
    x: f32,
    y: f32,
    width: f32,
}

impl Bar {
    pub fn new() -> Self {
        Self {
            x: (WIDTH - BAR_WIDTH) / 2.0,
            y: HEIGHT - BAR_HEIGHT - 10.0,
            width: BAR_WIDTH,
        }
    }

    pub fn move_left(&mut self) {
        self.x -= BAR_SPEED;
        if self.x < 0.0 {
            self.x = 0.0;
        }
    }

    pub fn move_right(&mut self) {
        self.x += BAR_SPEED;
        if self.x + self.width > WIDTH {
            self.x = WIDTH - self.width;
        }
    }
}
