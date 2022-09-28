use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub fn draw_rectangle(x: f64, y: f64, width: f64, height: f64, color: &str);
    fn set_score_text(text: &str);
}

#[wasm_bindgen]
#[derive(Default)]
pub struct Game {
    width: f64,
    height: f64,
    ball: Ball,
    p1: Player,
    p2: Player,
    started_physics: bool,
}

#[derive(Default)]
struct Player {
    r: Rect,
    keyup: bool,
    keydown: bool,
    score: usize,
}

#[derive(Default)]
struct Ball {
    r: Rect,
    vx: f64,
    vy: f64,
}

#[derive(Default)]
struct Rect {
    x: f64,
    y: f64,
    w: f64,
    h: f64,
}

#[wasm_bindgen]
impl Game {
    /// Creates a new game, initially frozen (no physics)
    pub fn new(width: f64, height: f64) -> Game {
        let mut game = Self {
            width,
            height,
            ..Default::default()
        };
        game.reset();
        game
    }

    /// Begin physics so the game can actually start
    pub fn start(&mut self) {
        self.started_physics = true;
    }

    /// Sets everything to the start of a round
    fn reset(&mut self) {
        self.p1.r = Rect {
            x: 120.,
            y: self.height / 2. - 60.,
            w: 20.,
            h: 120.,
        };
        self.p2.r = Rect {
            x: self.width - 120. - 20.,
            ..self.p1.r
        };
        self.ball.r = Rect {
            x: self.width / 2. - 10.,
            y: self.height / 2. - 10.,
            w: 20.,
            h: 20.,
        };
        self.ball.vx = 7.0_f64.copysign(self.ball.vx);
        self.ball.vy = 0.;
    }

    /// Called by javascript when a key gets pressed
    pub fn handle_keydown(&mut self, keycode: &str) {
        match keycode {
            "KeyW" => self.p1.keyup = true,
            "KeyS" => self.p1.keydown = true,
            "ArrowUp" => self.p2.keyup = true,
            "ArrowDown" => self.p2.keydown = true,
            _ => {}
        }
    }

    /// Called by javascript when a key gets released
    pub fn handle_keyup(&mut self, keycode: &str) {
        match keycode {
            "KeyW" => self.p1.keyup = false,
            "KeyS" => self.p1.keydown = false,
            "ArrowUp" => self.p2.keyup = false,
            "ArrowDown" => self.p2.keydown = false,
            _ => {}
        }
    }

    /// One iteration of the physics loop
    pub fn do_physics(&mut self) {
        if !self.started_physics {
            return;
        }
        respond_to_keyboard(&mut self.p1);
        respond_to_keyboard(&mut self.p2);
        self.ball.r.x += self.ball.vx;
        self.ball.r.y += self.ball.vy;
        if self.ball.r.y < 0. || self.ball.r.y > self.height - self.ball.r.h {
            self.ball.vy *= -1.;
        }
        collide(&mut self.ball, &self.p1.r);
        collide(&mut self.ball, &self.p2.r);
        if self.ball.r.x < 0. {
            self.p2.score += 1;
            self.reset();
            set_score_text(&format!("{} : {}", self.p1.score, self.p2.score));
        } else if self.ball.r.x + self.ball.r.h > self.width {
            self.p1.score += 1;
            self.reset();
            set_score_text(&format!("{} : {}", self.p1.score, self.p2.score));
        }
    }

    /// One iteration of the animation loop
    pub fn draw_animation_frame(&mut self) {
        draw_rectangle(0., 0., self.width, self.height, "#000000");
        draw(&self.ball.r);
        draw(&self.p1.r);
        draw(&self.p2.r);
    }
}

fn draw(r: &Rect) {
    draw_rectangle(r.x, r.y, r.w, r.h, "#FFFFFF");
}

fn respond_to_keyboard(p: &mut Player) {
    if p.keyup {
        p.r.y -= 6.;
    }
    if p.keydown {
        p.r.y += 6.;
    }
}

/// Checks for and handles a collision between the ball and a paddle 
fn collide(ball: &mut Ball, rect: &Rect) {
    if ball.r.x > rect.x + rect.w
        || rect.x > ball.r.x + ball.r.w
        || ball.r.y > rect.y + rect.h
        || rect.y > ball.r.y + ball.r.h
    {
        return;
    }
    let mag = (ball.vx * ball.vx + ball.vy * ball.vy).sqrt();
    let k = (ball.r.y + ball.r.h / 2. - rect.y - rect.h / 2.) / 30.;
    let d = 1. / (1. + k * k).sqrt();
    ball.vx = -1. * (mag * d).copysign(ball.vx);
    ball.vy = k * mag * d;
}
