use ggez::graphics::{Drawable, Font, Scale, Text, TextFragment};
use ggez::{
    Context, ContextBuilder, GameResult,
    conf::{Conf, WindowMode, WindowSetup},
    event::{self, EventHandler, KeyCode, KeyMods},
    graphics::{self, Color, DrawParam},
};
use rand::Rng;
use std::time::{Duration, Instant};

const WIDTH: f32 = 800.0;
const HEIGHT: f32 = 600.0;
const BALL_SIZE: f32 = 20.0;
const BAR_WIDTH: f32 = 150.0;
const BAR_HEIGHT: f32 = 20.0;
const BAR_SPEED: f32 = 10.0;
const BALL_SPEED: f32 = 5.0;
const INITIAL_HEARTS: usize = 3;

struct Ball {
    x: f32,
    y: f32,
    dx: f32,
    dy: f32,
}

struct Bar {
    x: f32,
    y: f32,
    width: f32,
}

struct Game {
    ball: Ball,
    bar: Bar,
    score: usize,
    high_score: usize,
    milestone: usize,
    hearts: usize,
    game_over: bool,
    paused: bool,
    font: Font,
    countdown_start: Option<Instant>,
    countdown_value: i32,
    fullscreen: bool,
}

impl Game {
    fn new(ctx: &mut Context) -> GameResult<Self> {
        let mut rng = rand::thread_rng();
        let font = Font::default();
        Ok(Self {
            ball: Ball {
                x: rng.gen_range(BALL_SIZE..WIDTH - BALL_SIZE),
                y: HEIGHT / 3.0,
                dx: if rng.gen_bool(0.5) {
                    BALL_SPEED
                } else {
                    -BALL_SPEED
                },
                dy: BALL_SPEED,
            },
            bar: Bar {
                x: (WIDTH - BAR_WIDTH) / 2.0,
                y: HEIGHT - BAR_HEIGHT - 10.0,
                width: BAR_WIDTH,
            },
            score: 0,
            high_score: 0,
            milestone: 10,
            hearts: INITIAL_HEARTS,
            game_over: false,
            paused: false,
            font,
            countdown_start: Some(Instant::now()),
            countdown_value: 5,
            fullscreen: false,
        })
    }

    fn reset(&mut self) {
        let mut rng = rand::thread_rng();
        self.ball.x = rng.gen_range(BALL_SIZE..WIDTH - BALL_SIZE);
        self.ball.y = HEIGHT / 3.0;
        self.ball.dx = if rng.gen_bool(0.5) {
            BALL_SPEED
        } else {
            -BALL_SPEED
        };
        self.ball.dy = BALL_SPEED;
        self.score = 0;
        self.hearts = INITIAL_HEARTS;
        self.game_over = false;
        self.countdown_start = Some(Instant::now());
        self.countdown_value = 5;
    }

    fn toggle_pause(&mut self) {
        self.paused = !self.paused;
    }

    fn toggle_fullscreen(&mut self, ctx: &mut Context) {
        self.fullscreen = !self.fullscreen;
        let mode = if self.fullscreen {
            WindowMode::default().fullscreen_type(ggez::conf::FullscreenType::True)
        } else {
            WindowMode::default().fullscreen_type(ggez::conf::FullscreenType::Windowed)
        };
        ggez::graphics::set_mode(ctx, mode).expect("Failed to toggle fullscreen");
    }

    fn update_countdown(&mut self) {
        if let Some(start_time) = self.countdown_start {
            if Instant::now().duration_since(start_time) >= Duration::from_secs(1) {
                self.countdown_value -= 1;
                self.countdown_start = Some(Instant::now());
            }
        }
    }
}

impl EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        if self.game_over {
            return Ok(());
        }

        if self.countdown_value > 0 {
            self.update_countdown();
            return Ok(());
        }

        if self.paused {
            return Ok(());
        }

        // Ball movement
        self.ball.x += self.ball.dx;
        self.ball.y += self.ball.dy;

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

            if self.score > self.high_score {
                self.high_score = self.score;
                self.hearts += 1;
                self.milestone += 5;
            }
        }

        // Ball falls off screen
        if self.ball.y > HEIGHT {
            self.hearts -= 1;
            if self.hearts == 0 {
                self.game_over = true;
            } else {
                self.reset();
            }
        }

        // Bar movement
        let keys = ggez::input::keyboard::pressed_keys(ctx);
        if keys.contains(&KeyCode::Left) || keys.contains(&KeyCode::A) {
            self.bar.x -= BAR_SPEED;
            if self.bar.x < 0.0 {
                self.bar.x = 0.0;
            }
        }
        if keys.contains(&KeyCode::Right) || keys.contains(&KeyCode::D) {
            self.bar.x += BAR_SPEED;
            if self.bar.x + self.bar.width > WIDTH {
                self.bar.x = WIDTH - self.bar.width;
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, Color::BLACK);
        

        // Draw countdown
        if self.countdown_value > 0 {
            let countdown_text = Text::new((
                format!("{}", self.countdown_value),
                self.font,
                // Scale::uniform(96.0),
                TextFragment::scale(self, 96.0),
            ));
            let dimensions = countdown_text.dimensions(ctx);
            graphics::draw(
                ctx,
                &countdown_text,
                DrawParam::default().dest([
                    WIDTH / 2.0 - dimensions.w / 2.0,
                    HEIGHT / 2.0 - dimensions.h / 2.0,
                ]),
            )?;
        } else {
            // Draw the ball
            let ball_rect = graphics::Rect::new(self.ball.x, self.ball.y, BALL_SIZE, BALL_SIZE);
            let ball = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                ball_rect,
                Color::WHITE,
            )?;
            graphics::draw(ctx, &ball, DrawParam::default())?;

            // Draw the bar
            let bar_rect = graphics::Rect::new(self.bar.x, self.bar.y, self.bar.width, BAR_HEIGHT);
            let bar = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                bar_rect,
                Color::GREEN,
            )?;
            graphics::draw(ctx, &bar, DrawParam::default())?;
        }

        graphics::present(ctx)
    }

    fn key_down_event(&mut self, ctx: &mut Context, keycode: KeyCode, _: KeyMods, _: bool) {
        match keycode {
            KeyCode::P | KeyCode::Space => self.toggle_pause(),
            KeyCode::F => self.toggle_fullscreen(ctx),
            KeyCode::R if self.game_over => self.reset(),
            _ => {}
        }
    }
}

fn main() -> GameResult {
    let (mut ctx, event_loop) = ContextBuilder::new("bounce_shield", "Your Name")
        .default_conf(Conf::new().window_mode(WindowMode::default().dimensions(WIDTH, HEIGHT)))
        .build()?;
    let game = Game::new(&mut ctx)?;
    event::run(ctx, event_loop, game)
}
