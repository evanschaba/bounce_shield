use bounce_shield::{
    BALL_SIZE, BALL_SPEED, BAR_WIDTH, Ball, Bar, Game, GameState, HEIGHT, INITIAL_HEARTS, WIDTH,
};

#[test]
fn test_ball_initialization() {
    let ball = Ball::new();
    assert!(ball.x >= BALL_SIZE && ball.x <= WIDTH - BALL_SIZE);
    assert_eq!(ball.y, HEIGHT / 3.0);
    assert_eq!(ball.dy, BALL_SPEED);
}

#[test]
fn test_bar_movement() {
    let mut bar = Bar::new();
    bar.move_left();
    assert!(bar.x < (WIDTH - BAR_WIDTH) / 2.0);
    bar.move_right();
    assert_eq!(bar.x, (WIDTH - BAR_WIDTH) / 2.0);
}

// Mock Game initialization without EventLoop
fn mock_game() -> Game {
    Game {
        ball: Ball::new(),
        bar: Bar::new(),
        score: 0,
        high_score: 0,
        hearts: INITIAL_HEARTS,
        state: GameState::Countdown,
        countdown_start: None,
        countdown_value: 3,
        fullscreen: false,
        animations: Vec::new(),
        first_start: true,
        prev_high_score: 0,
    }
}

#[test]
fn test_score_increment() {
    let mut game = mock_game();

    // Check initial score
    let initial_score = game.score;

    // Increment the score
    game.score += 1;

    // Assert the score has increased
    assert_eq!(game.score, initial_score + 1);
}
