extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use std::collections::VecDeque;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use graphics::*;
use piston::event_loop::{EventSettings, Events};
use piston::input::*;
use piston::window::WindowSettings;
use std::time::SystemTime;

type Point = [f64; 2];

pub struct App {
    clicked_point: VecDeque<(Point, SystemTime)>,
    recent_point: Point,
}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("Concentric Circle", [1000, 1000])
        .graphics_api(opengl)
        .fullscreen(true)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut gl = GlGraphics::new(opengl);
    let mut app = App {
        clicked_point: VecDeque::new(),
        recent_point: [0.0, 0.0],
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            gl.draw(r.viewport(), |c, g| {
                let transform = c.transform.trans(0.0, 0.0);

                clear(color::WHITE, g);

                let mut cnt = 0;

                for p in app.clicked_point.iter() {
                    let radius: f64 = p.1.elapsed().unwrap().as_millis() as f64;
                    if radius > 1000.0 {
                        cnt += 1;
                    } else {
                        let fadeout_color: [f32; 4] = [0., 0., 0., 1. - radius as f32 / 1000.];
                        circle_arc(fadeout_color, 5.0, 0.0, f64::_360() as f64 * 1.2,
                                    [p.0[0] - radius, p.0[1] - radius, radius * 2.0, radius * 2.0],
                                    transform, g);
                    }
                }
                for _ in 0..cnt {
                    app.clicked_point.pop_front();
                }
            });
        }

        if let Some(b) = e.press_args() {
            if let Button::Mouse(MouseButton::Left) = b {
                let new_point: (Point, SystemTime) = (app.recent_point, SystemTime::now());
                app.clicked_point.push_back(new_point);
            }
        }

        if let Some(m) = e.mouse_cursor_args() {
            app.recent_point = m;
        }
    }
}
