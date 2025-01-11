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

// Power-up types
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

// THESE WILL HAVE CHECKBOXES AND HUD TXT TO OFFER OPTIONS TO PLAY OR NAH
const AUDIO_PATH_GAME_BOUNCE: &str = "docs/assets/audio/game_bounce.wav"; // plays when ball hits the paddle 
const AUDIO_PATH_GAME_HEART: &str = "docs/assets/audio/game_heart.wav"; // plays when you earn a heart after hitting a high score(last round's score before u dropped the ball and retried + incremental milestone x10(every 10th+5 milestone))
const AUDIO_PATH_GAME_START: &str = "docs/assets/audio/game_start.wav"; // plays when at initial game start & when u press retry
const AUDIO_PATH_GAME_OVER: &str = "docs/assets/audio/game_over.wav"; // plays when you run out of hearts
const AUDIO_PATH_GAME_TUNE: &str = "docs/assets/audio/game_tune.wav"; // Soon i will make use of this, make some checkbox in the corner to enable/disable game background tune that loops while playing

impl Game {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        // Load all sound effects
        let hit_sound = audio::Source::new(ctx, AUDIO_PATH_GAME_BOUNCE).ok();
        let powerup_sound = audio::Source::new(ctx, AUDIO_PATH_GAME_POWERUP).ok();
        let heart_sound = audio::Source::new(ctx, AUDIO_PATH_GAME_HEART).ok();
        let start_sound = audio::Source::new(ctx, AUDIO_PATH_GAME_START).ok();
        let game_over_sound = audio::Source::new(ctx, AUDIO_PATH_GAME_OVER).ok();
        let mut background_music = audio::Source::new(ctx, AUDIO_PATH_GAME_TUNE).ok();

        // Set background music to loop
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

        // Play start sound
        if let Some(sound) = &mut game.start_sound {
            let _ = sound.play_detached(ctx);
        }

        // Start background music
        if let Some(music) = &mut game.background_music {
            let _ = music.play_detached(ctx);
        }

        game.add_animation(
            "Get Ready!".to_string(),
            [WIDTH / 2.0, HEIGHT / 2.0],
            2,
            72.0,
            Color::CYAN,
            [0.0, 0.0],
        );

        Ok(game)
    }

    fn apply_power_up(&mut self, power_up: &PowerUp, ctx: &Context) {
        match power_up.power_type {
            // Previous power-up implementations...
        }

        if let Some(sound) = &mut self.powerup_sound {
            let _ = sound.play_detached(ctx);
        }
    }

    pub fn reset(&mut self, ctx: &Context) {
        // Previous reset implementation...

        if let Some(sound) = &mut self.start_sound {
            let _ = sound.play_detached(ctx);
        }
    }

    pub fn check_high_score(&mut self, ctx: &Context) {
        if self.score > self.high_score {
            // Previous high score implementation...

            if !self.first_start && self.score > self.prev_high_score + 5 {
                self.hearts += 1;
                if let Some(sound) = &mut self.heart_sound {
                    let _ = sound.play_detached(ctx);
                }
                // Rest of the implementation...
            }
        }
    }

    pub fn handle_ball_collisions(&mut self, ctx: &Context) {
        // Ball-wall collision remains the same...

        // Ball-bar collision
        if self.ball.y + BALL_SIZE >= self.bar.y
            && self.ball.x + BALL_SIZE >= self.bar.x
            && self.ball.x <= self.bar.x + self.bar.width
        {
            // Previous collision handling...

            if let Some(sound) = &mut self.hit_sound {
                let _ = sound.play_detached(ctx);
            }

            self.check_high_score(ctx);
            self.spawn_power_up();
        }

        // Ball falls off screen
        if self.ball.y > HEIGHT {
            self.hearts -= 1;
            self.animations.clear();
            if self.hearts == 0 {
                if let Some(sound) = &mut self.game_over_sound {
                    let _ = sound.play_detached(ctx);
                }
                // Rest of game over implementation...
            }
            // Rest of implementation...
        }
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
                            self.add_animation(
                                format!("{}", self.countdown_value),
                                [WIDTH / 2.0, HEIGHT / 2.0 - 100.0],
                                1,
                                96.0,
                                Color::CYAN,
                                [0.0, 0.0],
                            );
                        } else {
                            self.state = GameState::Playing;
                            self.animations.clear();
                            self.add_animation(
                                "Game Start!".to_string(),
                                [WIDTH / 2.0, HEIGHT / 2.0 - 50.0],
                                2,
                                72.0,
                                Color::GREEN,
                                [0.0, -1.0],
                            );
                            self.add_animation(
                                "Press 'P' or SPACE to pause".to_string(),
                                [WIDTH / 2.0, HEIGHT / 2.0 + 50.0],
                                3,
                                24.0,
                                Color::WHITE,
                                [0.0, 0.0],
                            );
                        }
                    }
                }
                return Ok(());
            }
            GameState::Playing => {
                self.ball.update();
                self.handle_ball_collisions();
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
                    self.add_animation(
                        "PAUSED".to_string(),
                        [WIDTH / 2.0, HEIGHT / 2.0],
                        999,
                        72.0,
                        Color::CYAN,
                        [0.0, 0.0],
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

pub fn create_game_ctx() -> Result<(Context, ggez::event::EventLoop<()>), Box<dyn std::error::Error>>
{
    let mode = Conf::new().window_mode(WindowMode::default().dimensions(WIDTH, HEIGHT));
    let (ctx, event_loop) = ContextBuilder::new("bounce_shield", "🏐")
        .default_conf(mode)
        .build()?;
    Ok((ctx, event_loop))
}
