use ggez::ContextBuilder;
use ggez::audio::{self, SoundSource};
use ggez::conf::Conf;
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

// Constants
const WIDTH: f32 = 1920.0;
const HEIGHT: f32 = 1080.0;
const BALL_SIZE: f32 = 20.0;
const BAR_WIDTH: f32 = 150.0;
const BAR_HEIGHT: f32 = 20.0;
const INITIAL_BAR_SPEED: f32 = 10.0;
const INITIAL_BALL_SPEED: f32 = 5.0;
const INITIAL_HEARTS: usize = 3;
const DIFFICULTY_INCREASE_INTERVAL: usize = 5;
const POWERUP_SPAWN_CHANCE: f32 = 0.01;
const POWERUP_DURATION: Duration = Duration::from_secs(10);

// Audio paths
const AUDIO_PATH_GAME_BOUNCE: &str = "docs/assets/audio/game_bounce.wav";
const AUDIO_PATH_GAME_HEART: &str = "docs/assets/audio/game_heart.wav";
const AUDIO_PATH_GAME_START: &str = "docs/assets/audio/game_start.wav";
const AUDIO_PATH_GAME_OVER: &str = "docs/assets/audio/game_over.wav";
const AUDIO_PATH_GAME_TUNE: &str = "docs/assets/audio/game_tune.wav";
const AUDIO_PATH_GAME_POWERUP: &str = "docs/assets/audio/game_powerup.wav";

#[derive(Clone, Copy, PartialEq)]
enum PowerUpType {
    WidthIncrease,
    SpeedBoost,
    ExtraHeart,
    SlowBall,
}

struct PowerUp {
    x: f32,
    y: f32,
    power_type: PowerUpType,
    size: f32,
    active_until: Option<Instant>,
}

impl PowerUp {
    fn new(x: f32, y: f32, power_type: PowerUpType) -> Self {
        Self {
            x,
            y,
            power_type,
            size: 30.0,
            active_until: None,
        }
    }

    fn is_active(&self) -> bool {
        self.active_until
            .map(|time| Instant::now() < time)
            .unwrap_or(true)
    }
}

pub struct Bar {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub speed: f32,
    pub color: Color,
}

pub struct Ball {
    pub x: f32,
    pub y: f32,
    pub dx: f32,
    pub dy: f32,
    pub speed_multiplier: f32,
    pub color: Color,
}

pub struct AnimatedText {
    pub text: String,
    pub start_time: Instant,
    pub duration: Duration,
    pub position: [f32; 2],
    pub scale: f32,
    pub color: Color,
    pub velocity: [f32; 2],
}

impl AnimatedText {
    pub fn new(
        text: String,
        position: [f32; 2],
        duration_secs: u64,
        scale: f32,
        color: Color,
        velocity: [f32; 2],
    ) -> Self {
        Self {
            text,
            start_time: Instant::now(),
            duration: Duration::from_secs(duration_secs),
            position,
            scale,
            color,
            velocity,
        }
    }

    pub fn update(&mut self) {
        self.position[0] += self.velocity[0];
        self.position[1] += self.velocity[1];
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
    pub ball: Ball,
    pub bar: Bar,
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
    pub difficulty_level: usize,
    pub power_ups: Vec<PowerUp>,
    pub combo_count: usize,
    pub last_hit_time: Instant,
    pub hit_sound: Option<audio::Source>,
    pub powerup_sound: Option<audio::Source>,
    pub heart_sound: Option<audio::Source>,
    pub start_sound: Option<audio::Source>,
    pub game_over_sound: Option<audio::Source>,
    pub background_music: Option<audio::Source>,
}

impl Game {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let hit_sound = audio::Source::new(ctx, AUDIO_PATH_GAME_BOUNCE).ok();
        let powerup_sound = audio::Source::new(ctx, AUDIO_PATH_GAME_POWERUP).ok();
        let heart_sound = audio::Source::new(ctx, AUDIO_PATH_GAME_HEART).ok();
        let start_sound = audio::Source::new(ctx, AUDIO_PATH_GAME_START).ok();
        let game_over_sound = audio::Source::new(ctx, AUDIO_PATH_GAME_OVER).ok();
        let mut background_music = audio::Source::new(ctx, AUDIO_PATH_GAME_TUNE).ok();

        if let Some(music) = &mut background_music {
            music.set_repeat(true);
        }

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
            difficulty_level: 1,
            power_ups: Vec::new(),
            combo_count: 0,
            last_hit_time: Instant::now(),
            hit_sound,
            powerup_sound,
            heart_sound,
            start_sound,
            game_over_sound,
            background_music,
        };

        if let Some(sound) = &mut game.start_sound {
            let _ = sound.play_detached(ctx);
        }

        if let Some(music) = &mut game.background_music {
            let _ = music.play_detached(ctx);
        }

        game.add_start_animation();
        Ok(game)
    }

    fn add_start_animation(&mut self) {
        self.animations.push(AnimatedText::new(
            "Get Ready!".to_string(),
            [WIDTH / 2.0, HEIGHT / 2.0],
            2,
            72.0,
            Color::CYAN,
            [0.0, 0.0],
        ));
    }

    fn apply_power_up(&mut self, power_up: &PowerUp, ctx: &Context) {
        match power_up.power_type {
            PowerUpType::WidthIncrease => {
                self.bar.width *= 1.5;
            }
            PowerUpType::SpeedBoost => {
                self.bar.speed *= 1.5;
            }
            PowerUpType::ExtraHeart => {
                self.hearts += 1;
            }
            PowerUpType::SlowBall => {
                self.ball.speed_multiplier *= 0.75;
            }
        }

        if let Some(sound) = &mut self.powerup_sound {
            let _ = sound.play_detached(ctx);
        }
    }

    fn spawn_power_up(&mut self) {
        let mut rng = rand::thread_rng();
        if rng.gen::<f32>() < POWERUP_SPAWN_CHANCE {
            let power_type = match rng.gen_range(0..4) {
                0 => PowerUpType::WidthIncrease,
                1 => PowerUpType::SpeedBoost,
                2 => PowerUpType::ExtraHeart,
                _ => PowerUpType::SlowBall,
            };

            self.power_ups.push(PowerUp::new(
                rng.gen_range(0.0..WIDTH - 30.0),
                rng.gen_range(0.0..HEIGHT / 2.0),
                power_type,
            ));
        }
    }

    pub fn reset(&mut self, ctx: &Context) {
        self.ball = Ball::new();
        self.bar = Bar::new();
        self.score = 0;
        self.hearts = INITIAL_HEARTS;
        self.state = GameState::Countdown;
        self.countdown_start = Some(Instant::now());
        self.countdown_value = 3;
        self.animations.clear();
        self.power_ups.clear();
        self.combo_count = 0;
        self.difficulty_level = 1;

        self.add_start_animation();

        if let Some(sound) = &mut self.start_sound {
            let _ = sound.play_detached(ctx);
        }
    }

    pub fn check_high_score(&mut self, ctx: &Context) {
        if self.score > self.high_score {
            self.high_score = self.score;
            self.animations.push(AnimatedText::new(
                "New High Score!".to_string(),
                [WIDTH / 2.0, HEIGHT / 2.0 - 100.0],
                2,
                48.0,
                Color::YELLOW,
                [0.0, -1.0],
            ));

            if !self.first_start && self.score > self.prev_high_score + 5 {
                self.hearts += 1;
                if let Some(sound) = &mut self.heart_sound {
                    let _ = sound.play_detached(ctx);
                }
                self.animations.push(AnimatedText::new(
                    "+1 Heart!".to_string(),
                    [WIDTH / 2.0, HEIGHT / 2.0],
                    2,
                    48.0,
                    Color::GREEN,
                    [0.0, -1.0],
                ));
            }
        }
    }

    fn handle_bar_movement(&mut self, ctx: &Context) {
        let keyboard = ctx.keyboard;
        
        if keyboard.is_key_pressed(KeyCode::Left) || keyboard.is_key_pressed(KeyCode::A) {
            self.bar.move_left();
        }
        if keyboard.is_key_pressed(KeyCode::Right) || keyboard.is_key_pressed(KeyCode::D) {
            self.bar.move_right();
        }
    }

    pub fn handle_ball_collisions(&mut self, ctx: &Context) {
        // Ball-wall collisions
        if self.ball.x <= 0.0 || self.ball.x + BALL_SIZE >= WIDTH {
            self.ball.dx = -self.ball.dx;
        }
        if self.ball.y <= 0.0 {
            self.ball.dy = -self.ball.dy;
        }

        // Ball-bar collision
        if self.ball.y + BALL_SIZE >= self.bar.y
            && self.ball.x + BALL_SIZE >= self.bar.x
            && self.ball.x <= self.bar.x + self.bar.width
            && self.ball.dy > 0.0
        {
            self.ball.dy = -self.ball.dy;
            self.score += 1;
            
            // Update combo
            let hit_time = Instant::now();
            if hit_time.duration_since(self.last_hit_time) < Duration::from_secs(2) {
                self.combo_count += 1;
            } else {
                self.combo_count = 1;
            }
            self.last_hit_time = hit_time;

            // Play sound
            if let Some(sound) = &mut self.hit_sound {
                let _ = sound.play_detached(ctx);
            }

            self.check_high_score(ctx);
            self.spawn_power_up();

            // Increase difficulty
            if self.score % DIFFICULTY_INCREASE_INTERVAL == 0 {
                self.difficulty_level += 1;
                self.ball.speed_multiplier *= 1.1;
                self.bar.speed *= 1.05;
            }
        }

        // Ball falls off screen
        if self.ball.y > HEIGHT {
            self.hearts -= 1;
            self.animations.clear();
            if self.hearts == 0 {
                if let Some(sound) = &mut self.game_over_sound {
                    let _ = sound.play_detached(ctx);
                }
                self.state = GameState::GameOver;
                self.animations.push(AnimatedText::new(
                    "Game Over!".to_string(),
                    [WIDTH / 2.0, HEIGHT / 2.0 - 50.0],
                    999,
                    72.0,
                    Color::RED,
                    [0.0, 0.0],
                ));
                self.animations.push(AnimatedText::new(
                    "Press 'R' to retry".to_string(),
                    [WIDTH / 2.0, HEIGHT / 2.0 + 50.0],
                    999,
                    36.0,
                    Color::WHITE,
                    [0.0, 0.0],
                ));
            } else {
                self.ball = Ball::new();
                self.combo_count = 0;
            }
        }

        // Power-up collisions
        self.power_ups.retain_mut(|power_up| {
            let collided = self.ball.x < power_up.x + power_up.size
                && self.ball.x + BALL_SIZE > power_up.x
                && self.ball.y < power_up.y + power_up.size
                && self.ball.y + BALL_SIZE > power_up.y;

            if collided {
                self.apply_power_up(power_up, ctx);
                power_up.active_until = Some(Instant::now() + POWERUP_DURATION);
            }

            power_up.is_active()
        });
    }
} 


impl EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        // Update animations
        for anim in &mut self.animations {
            anim.update();
        }
        self.animations.retain(|anim| anim.is_active());

        match self.state {
            GameState::GameOver => return Ok(()),
            GameState::Paused => return Ok(()),
            GameState::Countdown => {
                if let Some(start_time) = self.countdown_start {
                    if Instant::now().duration_since(start_time) >= Duration::from_secs(1) {
                        self.countdown_value -= 1;
                        self.countdown_start = Some(Instant::now());
                        if self.countdown_value > 0 {
                            self.animations.clear();
                            self.animations.push(AnimatedText::new(
                                format!("{}", self.countdown_value),
                                [WIDTH / 2.0, HEIGHT / 2.0 - 100.0],
                                1,
                                96.0,
                                Color::CYAN,
                                [0.0, 0.0],
                            ));
                        } else {
                            self.state = GameState::Playing;
                            self.animations.clear();
                            self.animations.push(AnimatedText::new(
                                "Game Start!".to_string(),
                                [WIDTH / 2.0, HEIGHT / 2.0 - 50.0],
                                2,
                                72.0,
                                Color::GREEN,
                                [0.0, -1.0],
                            ));
                            self.animations.push(AnimatedText::new(
                                "Press 'P' or SPACE to pause".to_string(),
                                [WIDTH / 2.0, HEIGHT / 2.0 + 50.0],
                                3,
                                24.0,
                                Color::WHITE,
                                [0.0, 0.0],
                            ));
                        }
                    }
                }
                return Ok(());
            }
            GameState::Playing => {
                self.ball.update();
                self.handle_ball_collisions(ctx);
                self.handle_bar_movement(ctx);
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);

        // Draw score and game info
        let score_text = Text::new(
            TextFragment::new(format!(
                "Score: {}\nHearts: {}\nHigh Score: {}\nLevel: {}\nCombo: x{}",
                self.score, self.hearts, self.high_score, self.difficulty_level, self.combo_count
            ))
            .scale(24.0),
        );
        score_text.draw(&mut canvas, DrawParam::default().dest([20.0, 20.0]));

        // Draw controls
        let controls_text = Text::new(
            TextFragment::new(
                "Controls:\nF - Fullscreen\nP/Space - Pause\nR - Retry\nA/D or ←/→ - Move",
            )
            .scale(20.0),
        );
        controls_text.draw(
            &mut canvas,
            DrawParam::default().dest([WIDTH - 200.0, 20.0]),
        );

        // Draw game elements
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
            // Draw power-ups
            for power_up in &self.power_ups {
                let power_up_color = match power_up.power_type {
                    PowerUpType::WidthIncrease => Color::YELLOW,
                    PowerUpType::SpeedBoost => Color::RED,
                    PowerUpType::ExtraHeart => Color::GREEN,
                    PowerUpType::SlowBall => Color::CYAN,
                };

                let power_up_rect =
                    graphics::Rect::new(power_up.x, power_up.y, power_up.size, power_up.size);
                let power_up_mesh = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    power_up_rect,
                    power_up_color,
                )?;
                power_up_mesh.draw(&mut canvas, DrawParam::default());
            }

            // Draw ball
            let ball_rect = graphics::Rect::new(self.ball.x, self.ball.y, BALL_SIZE, BALL_SIZE);
            let ball = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                ball_rect,
                self.ball.color,
            )?;
            ball.draw(&mut canvas, DrawParam::default());

            // Draw bar
            let bar_rect = graphics::Rect::new(self.bar.x, self.bar.y, self.bar.width, BAR_HEIGHT);
            let bar = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                bar_rect,
                self.bar.color,
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
                    self.animations.push(AnimatedText::new(
                        "PAUSED".to_string(),
                        [WIDTH / 2.0, HEIGHT / 2.0],
                        999,
                        72.0,
                        Color::CYAN,
                        [0.0, 0.0],
                    ));
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
                ctx.gfx.set_mode(mode)?;
            }
            Some(KeyCode::R) if self.state == GameState::GameOver => {
                self.prev_high_score = self.high_score;
                self.reset(ctx);
            }
            _ => {}
        }
        Ok(())
    }
}

impl Ball {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            x: rng.gen_range(BALL_SIZE..WIDTH - BALL_SIZE),
            y: HEIGHT / 3.0,
            dx: if rng.gen_bool(0.5) {
                INITIAL_BALL_SPEED
            } else {
                -INITIAL_BALL_SPEED
            },
            dy: INITIAL_BALL_SPEED,
            speed_multiplier: 1.0,
            color: Color::WHITE,
        }
    }

    pub fn update(&mut self) {
        self.x += self.dx * self.speed_multiplier;
        self.y += self.dy * self.speed_multiplier;
    }
}

impl Bar {
    pub fn new() -> Self {
        Self {
            x: (WIDTH - BAR_WIDTH) / 2.0,
            y: HEIGHT - BAR_HEIGHT - 10.0,
            width: BAR_WIDTH,
            speed: INITIAL_BAR_SPEED,
            color: Color::GREEN,
        }
    }

    pub fn move_left(&mut self) {
        self.x -= self.speed;
        if self.x < 0.0 {
            self.x = 0.0;
        }
    }

    pub fn move_right(&mut self) {
        self.x += self.speed;
        if self.x + self.width > WIDTH {
            self.x = WIDTH - self.width;
        }
    }
}

pub fn create_game_ctx() -> Result<(Context, ggez::event::EventLoop<()>), Box<dyn std::error::Error>> {
    let mode = Conf::new().window_mode(WindowMode::default().dimensions(WIDTH, HEIGHT));
    let (ctx, event_loop) = ContextBuilder::new("bounce_shield", "🏐")
        .default_conf(mode)
        .build()?;
    Ok((ctx, event_loop))
}