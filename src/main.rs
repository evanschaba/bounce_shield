use minifb::{Key, Window, WindowOptions};
use rand::Rng;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;
const BALL_SIZE: f32 = 20.0;
const BAR_WIDTH: f32 = 150.0;
const BAR_HEIGHT: f32 = 20.0;
const BAR_SPEED: f32 = 8.0;
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
    hearts: usize,
    game_over: bool,
}

impl Game {
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            ball: Ball {
                x: rng.gen_range(BALL_SIZE..(WIDTH as f32 - BALL_SIZE)),
                y: HEIGHT as f32 / 3.0,
                dx: if rng.gen_bool(0.5) {
                    BALL_SPEED
                } else {
                    -BALL_SPEED
                },
                dy: BALL_SPEED,
            },
            bar: Bar {
                x: (WIDTH as f32 - BAR_WIDTH) / 2.0,
                y: HEIGHT as f32 - BAR_HEIGHT - 10.0,
                width: BAR_WIDTH,
            },
            score: 0,
            high_score: 0,
            hearts: INITIAL_HEARTS,
            game_over: false,
        }
    }

    fn reset(&mut self) {
        let mut rng = rand::thread_rng();
        self.ball = Ball {
            x: rng.gen_range(BALL_SIZE..(WIDTH as f32 - BALL_SIZE)),
            y: HEIGHT as f32 / 3.0,
            dx: if rng.gen_bool(0.5) {
                BALL_SPEED
            } else {
                -BALL_SPEED
            },
            dy: BALL_SPEED,
        };
        self.score = 0;
        self.hearts = INITIAL_HEARTS;
        self.game_over = false;
    }

    fn update_high_score(&mut self) {
        if self.score > self.high_score {
            self.high_score = self.score;
        }
    }
}

fn main() {
    let mut window = Window::new("bounce_shield v1.0.0", WIDTH, HEIGHT, WindowOptions {
        resize: false,
        scale: minifb::Scale::X1,
        ..WindowOptions::default()
    })
    .unwrap_or_else(|_| panic!("Failed to create window"));

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut game = Game::new();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        buffer.fill(0); // Clear screen

        if game.game_over {
            // Show "Game Over" screen
            if window.is_key_down(Key::R) {
                game.reset();
            }
        } else {
            // Ball movement
            game.ball.x += game.ball.dx;
            game.ball.y += game.ball.dy;

            // Ball-wall collision
            if game.ball.x <= 0.0 || game.ball.x + BALL_SIZE >= WIDTH as f32 {
                game.ball.dx *= -1.0;
            }
            if game.ball.y <= 0.0 {
                game.ball.dy *= -1.0;
            }

            // Ball-bar collision
            if game.ball.y + BALL_SIZE >= game.bar.y
                && game.ball.x + BALL_SIZE >= game.bar.x
                && game.ball.x <= game.bar.x + game.bar.width
            {
                game.ball.dy *= -1.0;
                game.score += 1;

                // Milestone: Only award hearts if a high score exists
                if game.score % 10 == 0 {
                    if game.score > game.high_score {
                        game.hearts += 1;
                    }
                    game.bar.width += 10.0;
                }
            }

            // Ball falls off the screen
            if game.ball.y > HEIGHT as f32 {
                game.hearts -= 1;
                if game.hearts == 0 {
                    game.game_over = true;
                    game.update_high_score();
                } else {
                    game.ball.y = HEIGHT as f32 / 3.0;
                    game.ball.dy = BALL_SPEED;
                }
            }

            // Bar movement
            if window.is_key_down(Key::Left) && game.bar.x > 0.0 {
                game.bar.x -= BAR_SPEED;
            }
            if window.is_key_down(Key::Right) && game.bar.x + game.bar.width < WIDTH as f32 {
                game.bar.x += BAR_SPEED;
            }
        }

        // Draw ball
        for i in 0..BALL_SIZE as usize {
            for j in 0..BALL_SIZE as usize {
                let x = (game.ball.x as usize + i).min(WIDTH - 1);
                let y = (game.ball.y as usize + j).min(HEIGHT - 1);
                buffer[y * WIDTH + x] = 0xFFFFFF;
            }
        }

        // Draw bar
        for i in 0..BAR_HEIGHT as usize {
            for j in 0..game.bar.width as usize {
                let x = (game.bar.x as usize + j).min(WIDTH - 1);
                let y = (game.bar.y as usize + i).min(HEIGHT - 1);
                buffer[y * WIDTH + x] = 0x00FF00;
            }
        }

        // Update the window
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap_or_else(|_| panic!("Failed to update buffer"));
    }
}
