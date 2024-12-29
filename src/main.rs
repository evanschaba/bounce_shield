
use bevy::prelude::*;
use bevy::app::Startup;

#[derive(Default)]
struct GameState;

fn setup_system() {
    // setup code here
}

fn countdown_system() {
    // countdown code here
}

fn game_over_system() {
    // game over code here
}

fn reward_punishment_system() {
    // reward/punishment code here
}

fn window_resize_system() {
    // window resize code here
}

fn restart_system() {
    // restart code here
}

fn main() {
    App::new()
        .add_systems(Startup, setup_system)
        // .add_system(StartupStage::Startup, setup_system)
        // .add_system(StartupStage::PreStartup, setup_system)
        // .add_systems(reward_punishment_system)
        // .add_systems(window_resize_system)
        // .add_systems(restart_system)
        .run();
}
