/**
* Copyright (c) AWildDevAppears
*/

use raylib::prelude::*;

struct Player {
    position: Vector2,
    velocity: Vector2,
}

#[derive(Clone, Copy)]
struct Projectile {
    position: Vector2,
    velocity: Vector2,
    alive: bool,
}

#[derive(Clone, Copy)]
struct Asteroid {
    position: Vector2,
    velocity: Vector2,
    level: i32, // 0 is destroyed
}


struct Keybinds {
    move_left: KeyboardKey,
    move_right: KeyboardKey,
    move_up: KeyboardKey,
    move_down: KeyboardKey,
    fire: KeyboardKey,
}

struct GameSettings {
    title: &'static str,
    width: i32,
    height: i32,
    player_max_velocity: Vector2,
    player_ticks_to_max_velocity: f32,
    max_projectiles: usize,
    max_asteroids: usize,
    keybinds: Keybinds,
}

impl GameSettings {
    fn bounds(&self) -> Vector2 {
        return Vector2 { x: self.width as f32, y: self.height as f32 };
    }
}

const GAME_SETTINGS: GameSettings = GameSettings {
    title: "Asteroids",
    width: 640,
    height: 640,
    player_max_velocity: Vector2 { x: 1.0, y: 1.0 },
    player_ticks_to_max_velocity: 5.0,
    max_projectiles: 30,
    max_asteroids: 10,
    keybinds: Keybinds {
        move_left: KeyboardKey::KEY_LEFT,
        move_right: KeyboardKey::KEY_RIGHT,
        move_up: KeyboardKey::KEY_UP,
        move_down: KeyboardKey::KEY_DOWN,
        fire: KeyboardKey::KEY_SPACE
    },
};

struct GameState {
    player: Player,
    projectiles: [Projectile; GAME_SETTINGS.max_projectiles],
    asteroids: [Asteroid; GAME_SETTINGS.max_asteroids],
    camera: Camera2D,
}

fn main() {
    let (mut game, thread) = raylib::init()
        .size(GAME_SETTINGS.width, GAME_SETTINGS.height)
        .title(GAME_SETTINGS.title)
        .build();

    let mut state = setup();

    while !game.window_should_close() {
        update(&mut state, &game);

        draw(&mut game, &thread, &mut state);
    }
}

fn setup() -> GameState {
    let player = Player { position: Vector2 { x: 50.0, y: 50.0 }, velocity: Vector2 { x: 0.0, y: 0.0 } };

    let blank_asteroid = Asteroid {
        position: Vector2 { x: 0.0, y: 0.0 },
        velocity: Vector2 { x: 0.0, y: 0.0 },
        level: 0,
    };

    let blank_projectile = Projectile {
        position: Vector2 { x: 0.0, y: 0.0 },
        velocity: Vector2 { x: 0.0, y: 0.0 },
        alive: false,
    };

    let camera = Camera2D {
        target: Vector2 { x: 0.0, y: 0.0 },
        offset: Vector2 { x: 0.0, y: 0.0 },
        rotation: 0.0,
        zoom: 1.0,
    };

    return GameState {
        player: player,
        projectiles: [blank_projectile; GAME_SETTINGS.max_projectiles],
        asteroids: [blank_asteroid; GAME_SETTINGS.max_asteroids],
        camera: camera,
    }
}

fn calculate_speed(speed: f32, max_speed: f32, time_to_max_speed: f32, invert: bool) -> f32 {
    let mult: f32 = if invert { -1.0 } else { 1.0 };
    let accel = (max_speed / time_to_max_speed) * mult;
    let new_speed = speed + accel;

    if invert {
        return new_speed.max(-max_speed);
    }

    return new_speed.min(max_speed);
}

fn update(state: &mut GameState, game: &RaylibHandle) {
    if game.is_key_down(GAME_SETTINGS.keybinds.move_left) {
        state.player.velocity.x = calculate_speed(
            state.player.velocity.x,
            GAME_SETTINGS.player_max_velocity.x,
            GAME_SETTINGS.player_ticks_to_max_velocity,
            true,
        );
    } else if game.is_key_down(GAME_SETTINGS.keybinds.move_right) {
        state.player.velocity.x = calculate_speed(
            state.player.velocity.x,
            GAME_SETTINGS.player_max_velocity.x,
            GAME_SETTINGS.player_ticks_to_max_velocity,
            false,
        );
    }

    state.player.position.x += state.player.velocity.x;
}

fn draw(game: &mut RaylibHandle, thread: &RaylibThread, state: &mut GameState) {
    let mut draw = game.begin_drawing(thread);

    draw.clear_background(Color::WHITESMOKE);
    {
        let mut draw = draw.begin_mode2D(state.camera);
        {
            draw.draw_rectangle_v(state.player.position, Vector2 { x: 10.0, y: 10.0 }, Color::RED);
        }
    }
}
