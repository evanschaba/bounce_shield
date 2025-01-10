# Game Details - bounce_shield

## Overview

**bounce_shield** is an exciting arcade-style game where players control a bar to bounce a ball, aiming to keep it in play and score points. The game includes a scoring system, a lives mechanic represented by hearts, and dynamic animations to enhance the experience.  

The goal is to keep the ball from falling off the screen while achieving high scores and unlocking rewards like extra hearts for milestones. With animated text effects and responsive gameplay, **bounce_shield** delivers a fun and engaging experience.  

## Features  

### Core Mechanics  

- **Ball Movement**:  
  The ball moves at a fixed speed, bouncing off the walls and the player's bar. Randomized initial directions add variety to gameplay.  

- **Bar Movement**:  
  Control the bar using **Left** and **Right** arrow keys (or **A** and **D**). The bar moves smoothly and stops at screen edges.  

- **Score System**:  
  Earn points with every successful bounce off the bar.  
  - The score resets when the game restarts.  
  - If the score exceeds the previous high score by 5 points, the player earns an extra heart.  

- **Lives (Hearts)**:  
  Players start with **3 hearts**. Each time the ball falls off the screen, a heart is lost. Losing all hearts results in **Game Over**.  

- **Game States**:  
  - **Countdown**: A brief countdown ("3, 2, 1, Go!") before gameplay starts.  
  - **Playing**: The main gameplay mode.  
  - **Paused**: Pause the game with **P** or **Spacebar**.  
  - **Game Over**: Displays a "Game Over" animation when all hearts are lost.  

### Animations  

- **Text Animations**:  
  Key messages (e.g., "Game Start!", "New High Score!", "Extra Heart Awarded!") are displayed with vibrant colors and fade after a short duration.  

- **Game Over Screen**:  
  Displays "Game Over!" and a message to retry using **R**.  

### Controls  

- **Arrow Keys / A, D**: Move the bar left or right.  
- **P or Spacebar**: Pause or resume gameplay.  
- **F**: Toggle fullscreen mode.  
- **R**: Restart after a Game Over.  

## Current Implementation  

- **Score Milestones**:  
  - Extra hearts are awarded when players surpass their previous high score by 5 points.  
  - Visual feedback is provided with animations like "Extra Heart Awarded!" or "New High Score!"  

- **Bar and Ball Collision**:  
  - The ball's direction changes dynamically based on its collision with the bar.  
  - Visual feedback and scoring make gameplay satisfying and interactive.  

- **Dynamic Countdown**:  
  A countdown appears at the beginning of each game to prepare the player.  

- **Visual Indicators**:  
  Score, hearts, and high score are displayed on-screen. Controls are shown in the top-right corner.  

## Planned Improvements  

1. **Enhanced Visual Effects**:  
   - Add collision effects like particle bursts or screen shakes.  

2. **Quirky Ball Behavior**:  
   - Introduce random ball behaviors, like speed boosts, changes in size, or warp gates at screen edges.  

3. **Power-ups**:  
   - Implement collectable power-ups, such as extra hearts or temporary bar enhancements.  

4. **User Profiles & Leaderboard**:  
   - Add support for player profiles and high-score tracking using a backend API.  

5. **Multiplayer Mode**:  
   - Explore a cooperative or competitive mode with another player.  

6. **UI Enhancements**:  
   - Improve in-game menus and overlays for a polished experience.  

7. **Sound Effects**:  
   - Add feedback for ball-bar collisions, score increases, and power-ups.  

---

© 2025 [Your Name]. All rights reserved.  

This software is released under the Unlicense. See the [UNLICENSE](LICENSE) for details.  