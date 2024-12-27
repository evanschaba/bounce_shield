use bounce_shield::{BALL_SIZE, BAR_HEIGHT, Game, SCREEN_HEIGHT, SCREEN_WIDTH};

#[test]
fn test_ball_reset_position() {
    let mut game = Game::new();
    let initial_x = game.ball_x;
    let initial_y = game.ball_y;

    game.reset_ball();

    assert_ne!(game.ball_x, initial_x);
    assert_ne!(game.ball_y, initial_y);
}

#[test]
fn test_ball_collision_with_walls() {
    let mut game = Game::new();
    game.ball_x = 0.0;
    game.ball_dx = -3.0;
    game.update();
    assert!(game.ball_dx > 0.0);

    game.ball_y = 0.0;
    game.ball_dy = -3.0;
    game.update();
    assert!(game.ball_dy > 0.0);

    game.ball_x = SCREEN_WIDTH - BALL_SIZE;
    game.ball_dx = 3.0;
    game.update();
    assert!(game.ball_dx < 0.0);
}

#[test]
fn test_ball_collision_with_bar() {
    let mut game = Game::new();
    game.ball_y = SCREEN_HEIGHT - BAR_HEIGHT - BALL_SIZE - 1.0;
    game.ball_x = game.bar_x + game.bar_width / 2.0 - BALL_SIZE / 2.0;
    game.update();
    assert!(game.ball_dy < 0.0);
    assert_eq!(game.score, 1);
}

#[test]
fn test_heart_loss_and_game_over() {
    let mut game = Game::new();
    game.hearts = 1;
    game.ball_y = SCREEN_HEIGHT - BAR_HEIGHT + 1.0;
    game.update();
    assert_eq!(game.hearts, 0);
    assert!(game.game_over);
    assert_eq!(game.text_animation, "GAME OVER!");
}

#[test]
fn test_restart_resets_game_state() {
    let mut game = Game::new();
    game.score = 10;
    game.hearts = 0;
    game.bar_width = 120.0;
    game.restart();
    assert_eq!(game.score, 0);
    assert_eq!(game.hearts, 3);
    assert_eq!(game.bar_width, 150.0);
}

#[test]
fn test_milestone_hearts_award() {
    let mut game = Game::new();
    game.score = 4;
    game.milestones = vec![5];
    game.ball_y = SCREEN_HEIGHT - BAR_HEIGHT - BALL_SIZE - 1.0;
    game.ball_x = game.bar_x + game.bar_width / 2.0 - BALL_SIZE / 2.0;
    game.update();
    assert_eq!(game.score, 5);
    assert_eq!(game.hearts, 4);
    assert!(game.bar_width > 150.0);
    assert!(!game.milestones.contains(&5));
}
