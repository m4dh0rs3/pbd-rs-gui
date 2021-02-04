use crate::math_util::Vec2D;

pub(crate) struct Sim {
    pub(crate) balls: Vec<Ball>,
    pub(crate) sticks: Vec<Stick>,

    pub(crate) speed: u8,
    pub(crate) resolve: u8,

    pub(crate) env: Env,
}

pub(crate) struct Env {
    pub(crate) dt: f64,

    pub(crate) g: Vec2D,
    pub(crate) air: f64,

    pub(crate) width: i32,
    pub(crate) height: i32,
}

#[derive(Clone)]
pub(crate) struct Ball {
    pub(crate) pos: Vec2D,
    pub(crate) vel: Vec2D,
    prv: Vec2D,

    pub(crate) mass: f64,
    inv: f64,

    ely: f64,
}

#[derive(Clone)]
pub(crate) struct Stick {
    pub(crate) i: usize,
    pub(crate) j: usize,

    pub(crate) len: f64,
    sfn: f64,
}

impl Ball {
    pub(crate) fn new(pos: Vec2D, vel: Vec2D, mass: f64, dt: f64) -> Ball {
        Ball {
            pos,
            vel,
            prv: pos + vel * dt,
            mass,
            inv: if mass == 0.0 { 0.0 } else { 1.0 / mass },
            ely: 1.0,
        }
    }

    fn step(&mut self, env: &Env) {
        self.vel = (self.prv - self.pos) / env.dt;
        self.in_box(env);

        self.pos = self.prv;

        self.vel += env.g * (env.dt * self.inv * env.air);

        self.prv = self.pos + (self.vel * env.dt);
    }

    fn in_box(&mut self, env: &Env) {
        if self.prv.x > env.width as f64 {
            self.prv.x = env.width as f64;
            self.vel.x = -self.vel.x.abs() * self.ely
        } else if self.prv.x < 0.0 {
            self.prv.x = 0.0;
            self.vel.x = self.vel.x.abs() * self.ely
        }

        if self.prv.y > env.height as f64 {
            self.prv.y = env.height as f64;
            self.vel.y = -self.vel.y.abs() * self.ely
        } else if self.prv.y < 0.0 {
            self.prv.y = 0.0;
            self.vel.y = self.vel.y.abs() * self.ely
        }
    }

    pub(crate) fn fix(&mut self) {
        self.mass = 0.0;
        self.inv = 0.0;
    }
}

impl Stick {
    pub(crate) fn new(i: usize, j: usize, len: f64) -> Stick {
        Stick {
            i,
            j,
            len,
            sfn: 1.0,
        }
    }

    fn project(&self, balls: &mut Vec<Ball>) {
        let ball = (balls[self.i].clone(), balls[self.j].clone());

        if !(ball.0.inv == 0.0 && ball.1.inv == 0.0) {
            let diff: Vec2D = ball.0.prv - ball.1.prv;
            let len = diff.abs();
            let off: Vec2D = (diff / len) * (len - self.len);

            balls[self.i].prv -= (off * (ball.0.inv / (ball.0.inv + ball.1.inv))) * self.sfn;
            balls[self.j].prv += (off * (ball.1.inv / (ball.0.inv + ball.1.inv))) * self.sfn;
        }
    }
}

impl Sim {
    fn new(speed: u8, resolve: u8, dt: f64, g: Vec2D, air: f64, width: i32, height: i32) -> Sim {
        Sim {
            speed,
            resolve,
            env: Env::new(dt, g, air, width, height),

            ..Default::default()
        }
    }

    pub(crate) fn insert(&mut self, ball: Ball) {
        self.balls.push(ball);
    }

    pub(crate) fn connect(&mut self, i: usize, j: usize, len: Option<f64>) {
        assert_ne!(i, j);

        self.sticks.push(Stick::new(
            i,
            j,
            match len {
                Some(len) => len,
                None => self
                    .balls
                    .get(i)
                    .unwrap()
                    .pos
                    .dist(&self.balls.get(j).unwrap().pos),
            },
        ));
    }

    pub(crate) fn step(&mut self) {
        for _w in 0..self.speed {
            for ball in self.balls.iter_mut() {
                ball.step(&self.env);
            }

            for _r in 0..self.resolve {
                for stick in self.sticks.iter_mut() {
                    stick.project(&mut self.balls);
                }
            }
        }
    }
}

impl Env {
    fn new(dt: f64, g: Vec2D, air: f64, width: i32, height: i32) -> Env {
        Env {
            dt,

            g,
            air,

            width,
            height,
        }
    }
}

impl Default for Sim {
    fn default() -> Self {
        Sim {
            balls: Vec::new(),
            sticks: Vec::new(),

            speed: 1u8,
            resolve: 1u8,

            env: Env::default(),
        }
    }
}

impl Default for Env {
    fn default() -> Self {
        Env {
            dt: 0.6,

            g: Vec2D::new(0.0, 9.81),
            air: 1.0,

            width: 960,
            height: 480,
        }
    }
}
