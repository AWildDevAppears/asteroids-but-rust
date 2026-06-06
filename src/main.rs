/**
* Copyright (c) AWildDevAppears
*/
use raylib::prelude::*;

struct Player {
    position: Vector2,
    velocity: Vector2,
    rotation: f32,
}

impl Player {
    fn get_traingle_points(&self) -> (Vector2, Vector2, Vector2) {
        let size = GAME_SETTINGS.player_size;
        let rot = self.rotation.to_radians();

        let p1 = Vector2 {
            x: self.position.x + size * rot.cos(),
            y: self.position.y + size * rot.sin(),
        };
        let p2 = Vector2 {
            x: self.position.x + size * (rot + 2.0 * std::f32::consts::PI / 3.0).cos(),
            y: self.position.y + size * (rot + 2.0 * std::f32::consts::PI / 3.0).sin(),
        };
        let p3 = Vector2 {
            x: self.position.x + size * (rot + 4.0 * std::f32::consts::PI / 3.0).cos(),
            y: self.position.y + size * (rot + 4.0 * std::f32::consts::PI / 3.0).sin(),
        };

        (p1, p2, p3)
    }
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
    shape_index: usize,
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
    player_size: f32,
    projectile_speed: f32,
    fire_delay: f32,
    max_projectiles: usize,
    max_asteroids: usize,
    keybinds: Keybinds,
    asteroid_velocity: f32,
}

impl GameSettings {
    fn bounds(&self) -> Vector2 {
        return Vector2 {
            x: self.width as f32,
            y: self.height as f32,
        };
    }
}

const GAME_SETTINGS: GameSettings = GameSettings {
    title: "Asteroids",
    width: 640,
    height: 640,
    player_max_velocity: Vector2 { x: 0.4, y: 0.4 },
    player_ticks_to_max_velocity: 5.0,
    player_size: 10.0,
    projectile_speed: 5.0,
    fire_delay: 0.5,
    max_projectiles: 30,
    max_asteroids: 10,
    keybinds: Keybinds {
        move_left: KeyboardKey::KEY_A,
        move_right: KeyboardKey::KEY_D,
        move_up: KeyboardKey::KEY_W,
        move_down: KeyboardKey::KEY_S,
        fire: KeyboardKey::KEY_SPACE,
    },
    asteroid_velocity: 0.1,
};

struct GameState {
    player: Player,
    projectiles: [Projectile; GAME_SETTINGS.max_projectiles],
    asteroids: [Asteroid; GAME_SETTINGS.max_asteroids],
    camera: Camera2D,
    fire_timer: f32,
    spawn_timer: f32,
}

const ASTEROID_SHAPE_1: [Vector2; 9] = [
    Vector2 { x: 0.0, y: 0.0 }, // Center
    Vector2 { x: -40.0, y: -20.0 },
    Vector2 { x: 10.0, y: -50.0 },
    Vector2 { x: 50.0, y: -30.0 },
    Vector2 { x: 40.0, y: 20.0 },
    Vector2 { x: 10.0, y: 50.0 },
    Vector2 { x: -30.0, y: 40.0 },
    Vector2 { x: -50.0, y: 10.0 },
    Vector2 { x: -40.0, y: -20.0 }, // Close the loop
];

const ASTEROID_SHAPE_2: [Vector2; 9] = [
    Vector2 { x: 0.0, y: 0.0 },
    Vector2 { x: -30.0, y: -40.0 },
    Vector2 { x: 30.0, y: -40.0 },
    Vector2 { x: 50.0, y: 0.0 },
    Vector2 { x: 30.0, y: 40.0 },
    Vector2 { x: -30.0, y: 40.0 },
    Vector2 { x: -50.0, y: 10.0 },
    Vector2 { x: -40.0, y: -10.0 },
    Vector2 { x: -30.0, y: -40.0 },
];

const ASTEROID_SHAPE_3: [Vector2; 9] = [
    Vector2 { x: 0.0, y: 0.0 },
    Vector2 { x: -20.0, y: -50.0 },
    Vector2 { x: 20.0, y: -50.0 },
    Vector2 { x: 40.0, y: -20.0 },
    Vector2 { x: 50.0, y: 20.0 },
    Vector2 { x: 20.0, y: 50.0 },
    Vector2 { x: -20.0, y: 50.0 },
    Vector2 { x: -50.0, y: 0.0 },
    Vector2 { x: -20.0, y: -50.0 },
];

const ASTEROID_SHAPES: &[&[Vector2]] = &[&ASTEROID_SHAPE_1, &ASTEROID_SHAPE_2, &ASTEROID_SHAPE_3];

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
    let player = Player {
        position: Vector2 { x: 50.0, y: 50.0 },
        velocity: Vector2 { x: 0.0, y: 0.0 },
        rotation: 0.0,
    };

    let blank_asteroid = Asteroid {
        position: Vector2 { x: 0.0, y: 0.0 },
        velocity: Vector2 { x: 0.0, y: 0.0 },
        level: 0,
        shape_index: 0,
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
        fire_timer: 0.0,
        spawn_timer: 2.0,
    };
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
    } else {
        let accel =
            GAME_SETTINGS.player_max_velocity.x / GAME_SETTINGS.player_ticks_to_max_velocity;
        if state.player.velocity.x > 0.0 {
            state.player.velocity.x = (state.player.velocity.x - accel).max(0.0);
        } else if state.player.velocity.x < 0.0 {
            state.player.velocity.x = (state.player.velocity.x + accel).min(0.0);
        }
    }

    state.player.position.x += state.player.velocity.x;

    if game.is_key_down(GAME_SETTINGS.keybinds.move_up) {
        state.player.velocity.y = calculate_speed(
            state.player.velocity.y,
            GAME_SETTINGS.player_max_velocity.y,
            GAME_SETTINGS.player_ticks_to_max_velocity,
            true,
        );
    } else if game.is_key_down(GAME_SETTINGS.keybinds.move_down) {
        state.player.velocity.y = calculate_speed(
            state.player.velocity.y,
            GAME_SETTINGS.player_max_velocity.y,
            GAME_SETTINGS.player_ticks_to_max_velocity,
            false,
        );
    } else {
        let accel =
            GAME_SETTINGS.player_max_velocity.y / GAME_SETTINGS.player_ticks_to_max_velocity;
        if state.player.velocity.y > 0.0 {
            state.player.velocity.y = (state.player.velocity.y - accel).max(0.0);
        } else if state.player.velocity.y < 0.0 {
            state.player.velocity.y = (state.player.velocity.y + accel).min(0.0);
        }
    }

    state.player.position.y += state.player.velocity.y;

    let mouse_pos = game.get_mouse_position();
    let dx = mouse_pos.x - state.player.position.x;
    let dy = mouse_pos.y - state.player.position.y;
    let dist_sq = dx * dx + dy * dy;

    if dist_sq > 1.0 {
        state.player.rotation = dy.atan2(dx).to_degrees();
    }

    if state.fire_timer > 0.0 {
        state.fire_timer -= game.get_frame_time();
    }

    if game.is_key_down(GAME_SETTINGS.keybinds.fire) && state.fire_timer <= 0.0 {
        let index = state.projectiles.iter().position(|p| !p.alive).unwrap_or(0);
        let rot = state.player.rotation.to_radians();
        state.projectiles[index] = Projectile {
            position: state.player.position,
            velocity: Vector2 {
                x: rot.cos() * GAME_SETTINGS.projectile_speed,
                y: rot.sin() * GAME_SETTINGS.projectile_speed,
            },
            alive: true,
        };
        state.fire_timer = GAME_SETTINGS.fire_delay;
    }

    for projectile in state.projectiles.iter_mut() {
        if projectile.alive {
            projectile.position.x += projectile.velocity.x;
            projectile.position.y += projectile.velocity.y;

            // Collision detection with asteroids
            for asteroid in state.asteroids.iter_mut() {
                if asteroid.level > 0 {
                    let dx = projectile.position.x - asteroid.position.x;
                    let dy = projectile.position.y - asteroid.position.y;
                    let dist_sq = dx * dx + dy * dy;

                    // Simple radius-based collision (assuming 40.0 base radius for asteroid)
                    let radius = asteroid.level as f32 * 20.0;
                    if dist_sq < radius * radius {
                        asteroid.level -= 1;
                        projectile.alive = false;
                        break;
                    }
                }
            }

            if projectile.alive
                && (projectile.position.x < 0.0
                    || projectile.position.x > GAME_SETTINGS.bounds().x
                    || projectile.position.y < 0.0
                    || projectile.position.y > GAME_SETTINGS.bounds().y)
            {
                projectile.alive = false;
            }
        }
    }

    state.spawn_timer -= game.get_frame_time();
    if state.spawn_timer <= 0.0 {
        if let Some(index) = state.asteroids.iter().position(|a| a.level == 0) {
            state.asteroids[index] = spawn_asteroid(game);
        }
        state.spawn_timer = 2.0;
    }

    for asteroid in state.asteroids.iter_mut() {
        if asteroid.level > 0 {
            asteroid.position.x += asteroid.velocity.x;
            asteroid.position.y += asteroid.velocity.y;

            let margin = 100.0;
            let off_screen = asteroid.position.x < -margin
                || asteroid.position.x > GAME_SETTINGS.width as f32 + margin
                || asteroid.position.y < -margin
                || asteroid.position.y > GAME_SETTINGS.height as f32 + margin;

            if off_screen {
                asteroid.level = 0;
            }
        }
    }
}

fn draw(game: &mut RaylibHandle, thread: &RaylibThread, state: &mut GameState) {
    let mut draw = game.begin_drawing(thread);

    draw.clear_background(Color::WHITESMOKE);
    {
        let mut draw = draw.begin_mode2D(state.camera);
        {
            let (p1, p2, p3) = state.player.get_traingle_points();
            draw.draw_triangle_lines(p1, p2, p3, Color::RED);

            for projectile in state.projectiles.iter() {
                if projectile.alive {
                    draw.draw_circle_v(projectile.position, 2.0, Color::BLACK);
                }
            }
        }
    }

    for asteroid in state.asteroids {
        if asteroid.level == 0 {
            continue;
        }
        let level_scale = asteroid.level as f32 / 3.0;
        let transformed_points: Vec<Vector2> = ASTEROID_SHAPES[asteroid.shape_index]
            .iter()
            .map(|p| Vector2 {
                x: asteroid.position.x + p.x * level_scale,
                y: asteroid.position.y + p.y * level_scale,
            })
            .collect();

        for i in 1..(transformed_points.len() - 1) {
            draw.draw_triangle(
                transformed_points[0],
                transformed_points[i + 1],
                transformed_points[i],
                Color::DARKGRAY,
            );
        }
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

#[derive(Clone, Copy)]
enum SpawnPoint {
    Top,
    Bottom,
    Left,
    Right,
}

const SPAWN_POINTS: [SpawnPoint; 4] = [
    SpawnPoint::Top,
    SpawnPoint::Bottom,
    SpawnPoint::Left,
    SpawnPoint::Right,
];

fn spawn_asteroid(game: &RaylibHandle) -> Asteroid {
    let side_index = game.get_random_value::<i32>(0..3);
    let side = SPAWN_POINTS[side_index as usize];

    let mut pos = Vector2 { x: 0.0, y: 0.0 };

    match side {
        SpawnPoint::Top => {
            pos.x = game.get_random_value::<i32>(0..GAME_SETTINGS.width - 1) as f32;
            pos.y = -GAME_SETTINGS.bounds().y / 10.0;
        }
        SpawnPoint::Right => {
            pos.x = GAME_SETTINGS.bounds().x + (GAME_SETTINGS.bounds().x / 10.0);
            pos.y = game.get_random_value::<i32>(0..GAME_SETTINGS.height - 1) as f32;
        }
        SpawnPoint::Bottom => {
            pos.x = game.get_random_value::<i32>(0..GAME_SETTINGS.width - 1) as f32;
            pos.y = GAME_SETTINGS.bounds().y + (GAME_SETTINGS.bounds().y / 10.0);
        }
        SpawnPoint::Left => {
            pos.x = -GAME_SETTINGS.bounds().x / 10.0;
            pos.y = game.get_random_value::<i32>(0..GAME_SETTINGS.height - 1) as f32;
        }
    }

    // Target a random point inside the screen to ensure it crosses the screen
    let target = Vector2 {
        x: game.get_random_value::<i32>(0..GAME_SETTINGS.width - 1) as f32,
        y: game.get_random_value::<i32>(0..GAME_SETTINGS.height - 1) as f32,
    };

    let diff = Vector2 {
        x: target.x - pos.x,
        y: target.y - pos.y,
    };
    let dist = (diff.x * diff.x + diff.y * diff.y).sqrt();
    let velocity = Vector2 {
        x: (diff.x / dist) * GAME_SETTINGS.asteroid_velocity,
        y: (diff.y / dist) * GAME_SETTINGS.asteroid_velocity,
    };

    Asteroid {
        position: pos,
        velocity: velocity,
        level: 3,
        shape_index: game.get_random_value::<i32>(0..(ASTEROID_SHAPES.len() - 1) as i32) as usize,
    }
}
