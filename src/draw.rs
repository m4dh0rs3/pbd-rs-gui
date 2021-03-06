use crate::pbd::*;
use macroquad::color::Color;
use macroquad::shapes::*;
use phf::phf_map;

pub(crate) static SCHEME: phf::Map<&'static str, Color> = phf_map! {
    "back" => Color::new(1.0, 1.0, 1.0, 1.0),
    "text" => Color::new(0.27, 0.27, 0.27, 1.0),
    "ball" => Color::new(0.27, 0.27, 0.27, 1.0),
    "stick" => Color::new(0.85, 0.85, 0.85, 1.0),
};

impl Sim {
    pub(crate) fn draw(&self) {
        for stick in &self.sticks {
            stick.draw(&self.balls);
        }

        for ball in &self.balls {
            ball.draw();
        }
    }
}

impl Ball {
    fn draw(&self) {
        draw_circle(
            self.pos.x as f32,
            self.pos.y as f32,
            self.mass as f32 * 0.5,
            *SCHEME.get("ball").unwrap(),
        );
    }
}

impl Stick {
    fn draw(&self, balls: &Vec<Ball>) {
        let balls = (
            balls.get(self.i).unwrap().pos,
            balls.get(self.j).unwrap().pos,
        );
        draw_line(
            balls.0.x as f32,
            balls.0.y as f32,
            balls.1.x as f32,
            balls.1.y as f32,
            3.0,
            *SCHEME.get("stick").unwrap(),
        );
    }
}
