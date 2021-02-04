use crate::pbd::*;
use crate::math_util::Vec2D;

pub(crate) enum RopeType {
    Loose,
    SingleFixed,
    Fixed,
}

impl Sim {
    pub(crate) fn rope(&mut self, origin: Vec2D, len: f64, detail: usize, rope_type: RopeType) {
        let i = self.balls.len();

        let k = len / detail as f64;

        self.insert(Ball::new(
            origin,
            Vec2D::new(0.0, 0.0), 
            match &rope_type {
                RopeType::Loose => 5.0,
                _ => 0.0,
            }, 
            0.0
        ));

        for j in 1..detail {
            self.insert(Ball::new(
                Vec2D::new(
                    origin.x + k * j as f64,
                    origin.y
                ),
                Vec2D::new(0.0, 0.0), 
                5.0, 
                0.0
            ));

            self.connect(i + j - 1, i + j, Some(k));
        }

        if let RopeType::Fixed = &rope_type {
            if let Some(last) = self.balls.last_mut() {
                last.fix();
            }
        }
    }

    pub(crate) fn tri(&mut self, a: Vec2D, b: Vec2D, c: Vec2D) {
        let i = self.balls.len();

        self.insert(Ball::new(a, Vec2D::new(0.0, 0.0), 5.0, 0.0));
        self.insert(Ball::new(b, Vec2D::new(0.0, 0.0), 5.0, 0.0));
        self.insert(Ball::new(c, Vec2D::new(0.0, 0.0), 5.0, 0.0));

        self.connect(i, i + 1, None);
        self.connect(i + 1, i + 2, None);
        self.connect(i, i + 2, None);
    }

    pub(crate) fn quad(&mut self, o: Vec2D, w: f64, h: f64) {
        let i = self.balls.len();

        self.insert(Ball::new(o, Vec2D::new(0.0, 0.0), 5.0, 0.0));
        self.insert(Ball::new(Vec2D::new(o.x + w, o.y), Vec2D::new(0.0, 0.0), 5.0, 0.0));
        self.insert(Ball::new(Vec2D::new(o.x + w, o.y + h), Vec2D::new(0.0, 0.0), 5.0, 0.0));
        self.insert(Ball::new(Vec2D::new(o.x, o.y + h), Vec2D::new(0.0, 0.0), 5.0, 0.0));

        self.connect(i, i + 1, None);
        self.connect(i + 1, i + 2, None);
        self.connect(i + 2, i + 3, None);
        self.connect(i, i + 3, None);
        self.connect(i + 0, i + 2, None);
        self.connect(i + 1, i + 3, None);
    }
}