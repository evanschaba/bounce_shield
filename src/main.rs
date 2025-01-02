use bevy::prelude::*;
// use bevy::text::cosmic_text::ttf_parser::Style;
// use bevy::window::WindowResized;
use ::rand::Rng;

const BALL_SIZE: f32 = 20.0;
// const BAR_HEIGHT: f32 = 20.0;
const INITIAL_BAR_WIDTH: f32 = 150.0;
const COUNTDOWN_DURATION: f32 = 3.0;

#[derive(Resource)]
pub struct Game {
    pub ball: Ball,
    pub(crate) b_bar: Bar,
    pub high_score: usize,
    pub score: usize,
    pub hearts: usize,
    pub milestones: Vec<usize>,
    pub game_over: bool,
    pub countdown: f32,
    pub text_animation: String,
    pub text_timer: f32,
    pub ball_speed_multiplier: f32,
    pub restart_msg_shown: bool,
}

#[derive(Component)]
pub struct Ball {
    pub x: f32,
    pub y: f32,
    pub dx: f32,
    pub dy: f32,
}

#[derive(Component)]
struct Bar {
    x: f32,
    width: f32,
}

#[derive(Resource)]
struct WindowSize {
    width: f32,
    height: f32,
}

impl Default for Game {
    fn default() -> Self {
        let mut rng = ::rand::thread_rng();
        Self {
            ball: Ball {
                x: rng.gen_range(BALL_SIZE..800.0 - BALL_SIZE),
                y: rng.gen_range(BALL_SIZE..600.0 / 2.0),
                dx: if rng.gen_bool(0.5) { 3.0 } else { -3.0 },
                dy: 3.0,
            },
            b_bar: Bar {
                x: (800.0 - INITIAL_BAR_WIDTH) / 2.0,
                width: INITIAL_BAR_WIDTH,
            },
            score: 0,
            high_score: 0,
            hearts: 3,
            milestones: vec![5],
            game_over: false,
            countdown: COUNTDOWN_DURATION,
            text_animation: "GAME START!".to_string(),
            text_timer: 0.0,
            ball_speed_multiplier: 1.0,
            restart_msg_shown: false,
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "bounce_shield".to_string(),
                resolution: (800.0, 600.0).into(),
                resizable: true,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(Game::default())
        .insert_resource(WindowSize {
            width: 800.0,
            height: 600.0,
        })
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        // .add_systems(Update, draw_ui)
        // .add_systems(Update, ball_collision_and_logic)
        // .add_systems(Update, resize_window)
        .run();
}

// fn main() {
//     App::new()
//         .add_plugins(DefaultPlugins.set(WindowPlugin {
//             primary_window: Some(Window {
//                 title: "bounce_shield".to_string(),
//                 resolution: (800.0, 600.0).into(),
//                 resizable: true,
//                 ..default()
//             }),
//             ..default()
//         }))
//         .add_plugins(DefaultPlugins)
//         .insert_resource(Game::default())
//         .insert_resource(WindowSize {
//             width: 800.0,
//             height: 600.0,
//         })
//         .add_systems(Startup, setup)
//         .add_systems(Update, update)
//         // .add_systems(Update, draw_ui)
//         // .add_systems(Update, ball_collision_and_logic)
//         // .add_systems(Update, resize_window)
//         .run();
// }

fn setup(mut cmds: Commands) {
    cmds.spawn(Camera2d::default());
}

fn update(
    mut game: ResMut<Game>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    window_size: Res<WindowSize>,
) {
    let dt = time.delta_secs();

    if game.countdown > 0.0 {
        game.countdown -= dt;
        game.text_animation = format!("Starting in: {:.0}", game.countdown.ceil());
        return;
    } else if game.countdown <= 0.0 && game.text_timer == 0.0 {
        game.text_timer = 1.0;
    }

    if game.text_timer > 0.0 {
        game.text_timer -= dt;
        return;
    }

    if game.game_over {
        if !game.restart_msg_shown {
            game.text_animation = "Press R to retry!".to_string();
            game.restart_msg_shown = true;
        }

        if keys.just_pressed(KeyCode::KeyR) {
            let mut rng = ::rand::thread_rng();
            *game = Game {
                ball: Ball {
                    x: rng.gen_range(BALL_SIZE..window_size.width - BALL_SIZE),
                    y: rng.gen_range(BALL_SIZE..window_size.height / 2.0),
                    dx: if rng.gen_bool(0.5) { 3.0 } else { -3.0 },
                    dy: 3.0,
                },
                b_bar: Bar {
                    x: (window_size.width - INITIAL_BAR_WIDTH) / 2.0,
                    width: INITIAL_BAR_WIDTH,
                },
                high_score: game.high_score,
                milestones: vec![game.high_score, game.high_score * 2],
                hearts: 3,
                ball_speed_multiplier: 1.0,
                countdown: COUNTDOWN_DURATION,
                ..Default::default()
            };
        }
        return;
    }

    // Ball movement
    game.ball.x += game.ball.dx * game.ball_speed_multiplier;
    game.ball.y += game.ball.dy * game.ball_speed_multiplier;

    // Ball wall collisions
    if game.ball.x <= 0.0 || game.ball.x + BALL_SIZE >= window_size.width {
        game.ball.dx *= -1.0;
    }

    if game.ball.y <= 0.0 {
        game.ball.dy *= -1.0;
    }

    // Bar movement
    if keys.pressed(KeyCode::ArrowLeft) && game.b_bar.x > 0.0 {
        game.b_bar.x -= 5.0;
    }
    if keys.pressed(KeyCode::ArrowRight) && game.b_bar.x + game.b_bar.width < window_size.width {
        game.b_bar.x += 5.0;
    }
}
