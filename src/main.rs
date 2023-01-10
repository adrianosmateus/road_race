use rusty_engine::{prelude::*, game};
use rand::prelude::*;

const PLAYER_SPEED: f32 = 250.0;
const ROAD_SPEED: f32 = 400.0;

struct GameState {
    health_amount: u8,
    lost: bool,
}

fn main() {
    let mut game = Game::new();

    // Create the player sprite
    let player_1 = game.add_sprite("player_1", SpritePreset::RacingCarBlue);
    player_1.translation.x = -500.0;
    player_1.layer = 10.0;
    player_1.collision = true;

    // Play the background music
    game.audio_manager.play_music(MusicPreset::WhimsicalPopsicle, 0.2);

    // Paint the road lines
    for i in 0..10 {
        let roadline = game.add_sprite(format!("roadline{}", i), SpritePreset::RacingBarrierWhite);
        roadline.scale = 0.5;
        println!("{}", i);
        roadline.translation.x = -600.0 + (150.0 * i as f32);
    }

    // Create obstacles
    let obstacles = vec![
        SpritePreset::RacingBarrelBlue,
        SpritePreset::RacingBarrelRed,
        SpritePreset::RacingConeStraight,
    ];
    for (i, preset) in obstacles.into_iter().enumerate() {
        let obstacle = game.add_sprite(format!("obstacle{}", i), preset);
        obstacle.layer = 10.0;
        obstacle.collision = true;
        obstacle.translation.x = thread_rng().gen_range(800.0..1600.0);
        obstacle.translation.y = thread_rng().gen_range(-350.0..350.0);
    }

    // Health message
    let health_message = game.add_text("health_text", "Health: 5");
    health_message.translation = Vec2::new(550.0, 320.0);

    game.add_logic(game_logic);
    game.run(GameState {
        health_amount: 5,
        lost: false
    });
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    // Quits if the game is over
    if game_state.lost {
        return;
    }

    // Read keyboard inputs
    let mut direction = 0.0;
    if engine.keyboard_state.pressed(KeyCode::Up) {
        direction += 1.0;
    }
    if engine.keyboard_state.pressed(KeyCode::Down) {
        direction -= 1.0;
    }

    // Move the car up and down
    let player_1 = engine.sprites.get_mut("player_1").unwrap();
    player_1.translation.y += direction * PLAYER_SPEED * engine.delta_f32;
    player_1.rotation = direction * 0.15; 
    if player_1.translation.y < -350.0 || player_1.translation.y > 250.0 {
        game_state.health_amount = 0;
    }

    // Move roadlines and obstacles
    for sprite in engine.sprites.values_mut() {
        if sprite.label.starts_with("roadline") {
            sprite.translation.x -= ROAD_SPEED * engine.delta_f32;
            // println!("{}", sprite.translation.x);
            if sprite.translation.x < -675.0 {
                sprite.translation.x += 1500.0;
            }
        }

        if sprite.label.starts_with("obstacle") {
            sprite.translation.x -= ROAD_SPEED * engine.delta_f32;
            if sprite.translation.x < -800.0 {
                sprite.translation.x = thread_rng().gen_range(800.0..1600.0);
                sprite.translation.y = thread_rng().gen_range(-350.0..350.0);
            }
        }
    }

    // Handle colisions
    let health_text = engine.texts.get_mut("health_text").unwrap();
    for event in engine.collision_events.drain(..) {
        
        println!("Just collided!");
        
        if !event.pair.either_contains("player_1") || event.state.is_end() {
            continue;
        }

        println!("Player collided! Remaining health: {}", game_state.health_amount);

        if game_state.health_amount > 0 {
            game_state.health_amount -= 1;
            health_text.value = format!("Health: {}", game_state.health_amount);
            engine.audio_manager.play_sfx(SfxPreset::Impact1, 0.2);
        }
    }

    // If the health reachs zero, the game stops and a game over advise appears
    if game_state.health_amount <= 0 {
        println!("Game over! Remaining health: {}", game_state.health_amount);

        game_state.lost = true;
        let game_over = engine.add_text("game_over", "Game Over!");
        game_over.font_size = 128.0;
        engine.audio_manager.stop_music();
        engine.audio_manager.play_sfx(SfxPreset::Jingle3, 0.5);
    }
}