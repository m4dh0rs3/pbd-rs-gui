use macroquad::prelude::*;

mod draw;
mod pbd;
mod shapes;
mod debug;
mod math_util;

use math_util::Vec2D;

use draw::SCHEME;

#[macroquad::main("PBD")]
async fn main() {
    //let font = load_ttf_font("assets/JetBrainsMono-Regular.ttf").await;

    let mut sim = pbd::Sim {
        speed: 2,

        env: pbd::Env {
            dt: 0.2,
            ..pbd::Env::default()
        },

        ..pbd::Sim::default()
    };

    sim.env.width = screen_width() as i32;
    sim.env.height = screen_height() as i32;

    let (width, height) = (sim.env.width as f64, sim.env.height as f64);

    sim.rope(
        Vec2D::new(
            width * 0.2,
            height * 0.1
        ),
        width * 0.6,
        20,
        shapes::RopeType::Fixed,
    );

    let j = sim.balls.len();

    sim.rope(
        Vec2D::new(
            width * 0.5, 
            height * 0.1
        ), 
        height * 0.4, 
        8, 
        shapes::RopeType::Loose,
    );

    sim.connect(j / 2, j, Some((height * 0.4) / 8.0));

    let j = sim.balls.len();

    sim.quad(Vec2D::new(width * 0.5, height * 0.5), width * 0.1, width * 0.1);

    sim.connect(j - 1, j, Some(0.0));

    loop {
        /* sim.env.width = screen_width() as i32;
        sim.env.height = screen_height() as i32;

        if is_mouse_button_pressed(MouseButton::Left) {
            let mouse = mouse_position();
            sim.insert(pbd::Ball::new(
                Vec2D::new(mouse.0, mouse.1),
                Vec2D::new(0.0, 0.0),
                10.0,
                0.0,
            ));
        } */

        sim.step();
        // sim.debug();

        clear_background(*SCHEME.get("back").unwrap());

        sim.draw();

        draw_text(
            &get_fps().to_string(),
            20.0,
            10.0,
            24.0,
            *SCHEME.get("text").unwrap(),
        );

        /* draw_text_ex(
            &get_fps().to_string(),
            20.0,
            10.0,
            TextParams {
                font: font,
                font_size: 48,
                font_scale: 0.4,
                color: *SCHEME.get("text").unwrap(),
            },
        ); */

        next_frame().await
    }
}
