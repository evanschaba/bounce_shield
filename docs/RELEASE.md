# bounce_shield v1.1.0

![Test Status](https://github.com/evanschaba/bounce_shield/actions/workflows/ci_unit_test.yml/badge.svg)

## Overview

**bounce_shield** is a captivating arcade game where the player controls a bar to bounce a ball and prevent it from falling off the screen. It combines a straightforward gameplay loop with engaging mechanics like a scoring system, hearts for lives, and milestone bonuses. The game is developed using [ggez](https://github.com/ggez/ggez), a lightweight Rust game engine, and adheres to modern design and development principles.

## Features

### Gameplay Mechanics
- **Ball Physics**: The ball bounces off walls and the player's bar. It moves at a fixed speed but can exhibit dynamic behaviors in future versions.
- **Bar Movement**: The player uses the **left** and **right** arrow keys (or **A/D**) to control the bar.
- **Score System**: Players earn points each time the ball hits the bar. Milestone rewards include extra hearts or other bonuses.
- **Hearts**: Start with **3 hearts**. Losing the ball reduces hearts, and the game ends when all hearts are lost.
- **Countdown Start**: A countdown timer builds anticipation at the start of each round.
- **Animations**: Smooth and dynamic text animations for milestones, game state changes, and events.

### Game States
- **Countdown**: A 3-second timer before gameplay starts.
- **Playing**: The main gameplay loop.
- **Paused**: The game can be paused using **P** or **Space**.
- **Game Over**: Displays a "GAME OVER" screen when all hearts are lost, allowing the player to press **R** to restart.

### Visuals & Controls
- **Text Animations**: Animated messages (e.g., "Get Ready!", "Extra Heart Awarded!") enhance the gameplay experience.
- **HUD**: Displays the current score, hearts, and high score.
- **Controls**:
  - **Arrow Keys (or A/D)**: Move the bar.
  - **P/Space**: Pause/unpause the game.
  - **F**: Toggle fullscreen mode.
  - **R**: Restart the game after game over.

### Technical Implementation
- Built with [ggez](https://github.com/ggez/ggez), ensuring performance and flexibility.
- Implements modular and reusable Rust structs for `Ball`, `Bar`, and `Game`.
- Uses `rand` for randomized ball spawning.
- Clean, scalable architecture suitable for future expansions.

---

## What's New in v1.0.0
This release introduces the initial implementation of **bounce_shield**, featuring all essential mechanics and systems required for a fun and replayable arcade game.

### New Features
- **Animated Text Effects**: Dynamic messages for milestones and events.
- **Game States**: Fully implemented states (Countdown, Playing, Paused, Game Over).
- **High Score Tracking**: Automatically tracks the highest score achieved.
- **Milestone Bonuses**: Awards extra hearts when surpassing certain score thresholds.
- **Fullscreen Toggle**: Switch between fullscreen and windowed mode with **F**.
- **Smooth Bar and Ball Movement**: Responsive and intuitive controls with clear collision handling.

### Improvements Since Pre-release
- **Bug Fixes**: Resolved inconsistencies in collision detection and scoring logic.
- **Enhanced Countdown Mechanic**: Added animations and transitions for a smoother game start.
- **Visual Feedback**: Added "PAUSED" and "GAME OVER" messages for better user experience.
- **Hearts Notification**: Displays "Lost a heart!" or "Extra Heart Awarded!" messages.
- **Dynamic Ball Spawning**: Improved randomness in ball spawn location and movement direction.

---

## Future Plans
- **User Profiles & Leaderboards**: Integrate user management and online/offline leaderboards.
- **Collision Effects**: Add particle effects and screen shakes to enhance immersion.
- **Ball Quirks**: Introduce randomized ball behaviors (e.g., speed changes, color shifts).
- **Power-ups**: Implement collectibles for temporary boosts or abilities.
- **Multiplayer Mode**: Add support for competitive or cooperative multiplayer.
- **Enhanced UI**: Polish the HUD and add menu screens for settings and player selection.

---

## Known Issues
- **Bar-Movement Limitation**: The bar may momentarily "stick" if moved quickly to the screen edges.
- **Ball Speed**: Fixed speed may become repetitive; future updates will address this with variable speeds.
- **Limited Animations**: Text animations are basic; future versions will expand their complexity.

---

## Installation & Running

To play the game, ensure you have Rust installed. Then clone the repository and run the following:

```zsh

# clone
git clone https://github.com/evanschaba/bounce_shield.git

# navigate to cloned project
cd bounce_shield

# run the game

cargo run

# For continuous development and testing:
cargo watch -x check -x test -x run
```

Enjoy bounce_shield v1.0.0 and start bouncing your way to high scores!