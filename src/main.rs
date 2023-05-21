use macroquad::audio::{load_sound, play_sound, stop_sound, PlaySoundParams, Sound};
use macroquad::prelude::*;
use std::collections::HashMap;

struct EngineSound {
    running: bool,
    sound: Option<Sound>,
}

impl EngineSound {
    fn start(&mut self) {
        if !self.running {
            let params = PlaySoundParams {
                looped: true,
                volume: 1.0,
            };
            play_sound(self.sound.unwrap(), params);
        }
        self.running = true;
    }

    fn stop(&mut self) {
        if self.running {
            stop_sound(self.sound.unwrap());
        }
        self.running = false;
    }
}

struct Car {
    position: Vec2,
    rotation: f64,
    velocity: f64,
    max_velocity: f64,
    turn_speed: f64,
    acceleration: f64,
    breaking_speed: f64,
    resistance: f64,
    engine_sound: EngineSound,
}

impl Car {
    fn new() -> Car {
        Car {
            position: Vec2::new(screen_width() / 2., screen_height() / 2.),
            rotation: 90.0_f64.to_radians(),
            velocity: 0.,
            max_velocity: 200.,
            turn_speed: 3.,
            acceleration: 500.,
            breaking_speed: 500.,
            resistance: 200.,
            engine_sound: EngineSound {
                running: false,
                sound: None,
            },
        }
    }

    fn update(&mut self, delta: f64) {
        self.velocity -= self.resistance * delta;
        if self.velocity < 0. {
            self.velocity = 0.;
        }

        self.position.x += (self.rotation.sin() * self.velocity * delta) as f32;
        self.position.y += (self.rotation.cos() * -1. * self.velocity * delta) as f32;

        if self.position.x < 0. {
            self.position.x = 0.;
        }

        if self.position.x > screen_width() {
            self.position.x = screen_width();
        }

        if self.position.y < 0. {
            self.position.y = 0.;
        }

        if self.position.y > screen_height() {
            self.position.y = screen_height();
        }
    }

    fn turn_left(&mut self, delta: f64) {
        self.rotation -= self.turn_speed * delta;
    }

    fn turn_right(&mut self, delta: f64) {
        self.rotation += self.turn_speed * delta;
    }

    fn accelerate(&mut self, delta: f64) {
        self.velocity += self.acceleration * delta;
        if self.velocity > self.max_velocity {
            self.velocity = self.max_velocity;
        }
    }

    fn decelerate(&mut self, delta: f64) {
        self.velocity -= self.breaking_speed * delta;
    }

    fn draw(&self, game: &Game) {
        let car_texture = *game.textures.get("bil1").unwrap();

        let car_scale = 1.;

        let params = DrawTextureParams {
            dest_size: Some(Vec2::new(
                car_texture.width() * car_scale,
                car_texture.height() * car_scale,
            )),
            source: None,
            rotation: self.rotation as f32 - 90.0_f64.to_radians() as f32,
            flip_x: true,
            flip_y: false,
            pivot: None,
        };

        draw_texture_ex(
            car_texture,
            self.position.x - car_texture.width() / 2. * car_scale,
            self.position.y - car_texture.height() / 2. * car_scale,
            WHITE,
            params,
        );
    }
}

enum State {
    TITLE,
    GAME,
}

struct Game<'a> {
    state: State,
    car1: Car,
    car2: Car,
    textures: HashMap<&'a str, Texture2D>,
    sounds: HashMap<&'a str, Sound>,
    last_ts: f64,
    delta: f64,
}

impl<'a> Game<'a> {
    fn new() -> Game<'a> {
        Game {
            state: State::GAME,
            car1: Car::new(),
            car2: Car::new(),
            textures: HashMap::new(),
            sounds: HashMap::new(),
            last_ts: get_time(),
            delta: 0.,
        }
    }

    fn new_frame(&mut self) {
        let now_ts = get_time();
        let delta = now_ts - self.last_ts;
        self.delta = delta;
        self.last_ts = now_ts;
    }
}

fn draw_text_center_aligned(text: &str, y: f32, font_size: f32, font_color: Color) {
    let text_size = measure_text(text, None, font_size as _, 1.0);
    draw_text(
        text,
        screen_width() / 2. - text_size.width / 2.,
        y - text_size.height / 2.,
        font_size,
        font_color,
    );
}

fn handle_state_title() -> State {
    let mut text_color = YELLOW;

    if is_key_down(KeyCode::Space) {
        text_color = PINK;
    }

    let font_size = 35.;

    clear_background(BLACK);

    draw_text_center_aligned("BIL", 100., font_size, text_color);

    draw_text_center_aligned("ETT SPEL AV:", 150., font_size, text_color);

    draw_text_center_aligned("ALICE DAVID JULIA PAPPA MAMMA", 200., font_size, text_color);

    draw_text_center_aligned("TRYCK [ENTER] FÖR ATT SPELA", 300., font_size, text_color);

    if is_key_down(KeyCode::Enter) {
        return State::GAME;
    }

    return State::TITLE;
}

fn handle_state_game(game: &mut Game) -> State {
    game.car1.update(game.delta);
    game.car2.update(game.delta);

    clear_background(WHITE);

    if is_key_down(KeyCode::Right) {
        game.car1.turn_right(game.delta);
    }
    if is_key_down(KeyCode::Left) {
        game.car1.turn_left(game.delta);
    }
    if is_key_down(KeyCode::Down) {
        game.car1.decelerate(game.delta);
    }
    if is_key_down(KeyCode::Up) {
        game.car1.accelerate(game.delta);
        game.car1.engine_sound.start();
    } else {
        game.car1.engine_sound.stop();
    }

    if is_key_down(KeyCode::D) {
        game.car2.turn_right(game.delta);
    }
    if is_key_down(KeyCode::A) {
        game.car2.turn_left(game.delta);
    }
    if is_key_down(KeyCode::S) {
        game.car2.decelerate(game.delta);
    }
    if is_key_down(KeyCode::W) {
        game.car2.accelerate(game.delta);
        game.car2.engine_sound.start();
    } else {
        game.car2.engine_sound.stop();
    }

    let bg_texture = *game.textures.get("bg1").unwrap();

    let params = DrawTextureParams {
        dest_size: Some(Vec2::new(screen_width(), screen_height())),
        source: None,
        rotation: 0.,
        flip_x: false,
        flip_y: false,
        pivot: None,
    };

    draw_texture_ex(bg_texture, 0., 0., WHITE, params);

    game.car1.draw(&game);
    game.car2.draw(&game);

    return State::GAME;
}

#[macroquad::main("Bil")]
async fn main() {
    let mut game = Game::new();

    game.textures
        .insert("bil0", load_texture("assets/bil0.png").await.unwrap());
    game.textures
        .insert("bil1", load_texture("assets/bil1.png").await.unwrap());
    game.textures
        .insert("bg0", load_texture("assets/bg0.png").await.unwrap());
    game.textures
        .insert("bg1", load_texture("assets/bg1.png").await.unwrap());
    game.sounds.insert(
        "car_engine_p1",
        load_sound("assets/sounds/car_engine.wav").await.unwrap(),
    );
    game.sounds.insert(
        "car_engine_p2",
        load_sound("assets/sounds/car_engine.wav").await.unwrap(),
    );

    game.car1.engine_sound.sound = Some(*game.sounds.get("car_engine_p1").unwrap());
    game.car2.engine_sound.sound = Some(*game.sounds.get("car_engine_p2").unwrap());

    loop {
        game.new_frame();

        print!("delta: {}\n", game.delta);

        print!(
            "car1 x: {} y: {} vel: {}, rot: {}\n",
            game.car1.position.x,
            game.car1.position.y,
            game.car1.velocity,
            game.car1.rotation.to_degrees()
        );

        print!(
            "car2 x: {} y: {} vel: {}, rot: {}\n",
            game.car2.position.x,
            game.car2.position.y,
            game.car2.velocity,
            game.car2.rotation.to_degrees()
        );

        if is_key_down(KeyCode::Escape) {
            return;
        }

        match game.state {
            State::TITLE => {
                game.state = handle_state_title();
            }
            State::GAME => {
                game.state = handle_state_game(&mut game);
            }
        }

        next_frame().await
    }
}
