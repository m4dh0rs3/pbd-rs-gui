use crate::{math_util::Vec2D, pbd::*};

impl Sim {
    pub(crate) fn debug(&self) {
        println!(
            "\tSimulator:\n-#Balls: {}\n-#Sticks: {}",
            self.balls.len(),
            self.sticks.len()
        );

        println!("Balls: ");

        for ball in &self.balls {
            ball.debug();
        }

        /* println!("Sticks: ");

        for stick in &self.sticks {
            stick.debug();
        } */
    }
}

impl Ball {
    pub(crate) fn debug(&self) {
        println!("p: {}, v: {}", self.pos, self.vel);
    }
}

impl Stick {
    pub(crate) fn debug(&self) {
        println!("i: {}, j: {}, l: {}", self.i, self.j, self.len);
    }
}

impl std::fmt::Display for Vec2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}]", self.x, self.y)
    }
}