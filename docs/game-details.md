# Game Details - BounceShield

## Overview

**BounceShield** is an engaging and simple arcade game where the player controls a bar to bounce a ball and prevent it from falling off the screen. The game includes a basic scoring system, hearts that represent lives, and milestones that grant the player more hearts or increase the size of the bar for added challenge.

## Features

- **Ball Movement**: The ball bounces off the screen walls and the player's bar. It moves at a fixed speed, but future versions may include random speed changes or quirky behaviors.
- **Bar Movement**: The player controls a bar at the bottom of the screen using the **left** and **right** arrow keys.
- **Score System**: The player earns points by bouncing the ball off the bar. Milestones are used to grant extra hearts and increase the bar size.
- **Hearts**: The player starts with 3 hearts. Each time the ball falls off the screen, a heart is lost. If all hearts are lost, the game ends.
- **Game Over**: The game will display a "GAME OVER" screen when the player runs out of hearts, and they can press **R** to restart the game.

## Current Mechanics

- **Ball Bounces** off the walls and bar, with the ball's speed and direction changing upon collision.
- **Player Bar** can be moved left and right to keep the ball in play.
- **Score** is displayed and increases each time the ball hits the bar.
- **Hearts** are represented as a counter, and are reduced when the ball falls off the screen.
- **Milestones** trigger additional hearts and bar size increases after a certain score is reached.

## Future Improvements

- **User Management & Leaderboard**:
    - A UI to manage player profiles and show high scores.
    - A self-hosted backend API to track scores and players.

- **Animated Collision Effects**:
    - Add visual effects for ball and bar collisions, such as screen shake or flashing colors.

- **Ball Quirks**:
    - Introduce random behaviors, like dimming the ball to make it harder to follow, or turning some walls into warp gates to add a twist to gameplay.

- **Color & Visual Enhancements**:
    - Add more colorful visuals to enhance the overall look and feel of the game.

- **Power-ups**:
    - Implement power-ups such as extra hearts or speed boosts to make the gameplay more dynamic.

## Technical Details

- **Game Engine**: Built with [macroquad](https://github.com/not-fl3/macroquad), a Rust game engine for fast prototyping.
- **Controls**:
    - **Left Arrow**: Move the bar left.
    - **Right Arrow**: Move the bar right.
    - **R**: Restart the game after game over.

## Planned Features

- **UI Interface** for managing user profiles and showing high scores.
- **Animated Effects** for ball-bar collisions and other game events.
- **Quirky Ball Behavior** to add more unpredictability to the game.
- **Multiplayer Mode** or **Online Leaderboards** in the future.

---

© 2024 [Your Name]. All rights reserved.

This software is released under the Unlicense. See the [UNLICENSE](UNLICENSE) for details.