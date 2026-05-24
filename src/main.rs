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

struct GameSettings {
    title: &'static str,
    width: i32,
    height: i32,
    player_max_velocity: Vector2,
    max_projectiles: usize,
    max_asteroids: usize,
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
    max_projectiles: 30,
    max_asteroids: 10,
};

struct GameState {
    player: Player,
    projectiles: [Projectile; GAME_SETTINGS.max_projectiles],
    asteroids: [Asteroid; GAME_SETTINGS.max_asteroids],
    camera: Camera2D,
}

fn main() {
    let (game, thread) = raylib::init()
        .size(GAME_SETTINGS.width, GAME_SETTINGS.height)
        .title(GAME_SETTINGS.title)
        .build();

    let mut state = setup();

    while !game.window_should_close() {
        update(&state);

        draw(game, thread);
    }
}

fn setup() -> GameState {
    let player = Player { position:Vector2 { x: 0.0, y: 0.0 }, velocity: Vector2 { x: 0.0, y: 0.0 } };

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
        zoom: 0.0,
    };

    return GameState {
        player: player,
        projectiles: [blank_projectile; GAME_SETTINGS.max_projectiles],
        asteroids: [blank_asteroid; GAME_SETTINGS.max_asteroids],
        camera: camera,
    }
}

fn update(state: &GameState) {

}

fn draw(mut game: RaylibHandle, thread: RaylibThread) {
    let mut draw = game.begin_drawing(&thread);

    draw.clear_background(Color::WHITESMOKE);
    {

    }
}
