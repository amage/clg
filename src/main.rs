extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;
use rand::Rng;

const WIDTH: usize = 320;
const HEIGHT: usize = 240;

pub struct App {
    field: [i32; WIDTH * HEIGHT],
    run: bool,
    time: f64,
    gl: GlGraphics,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BACKGROUND: [f32; 4] = [0.1, 0.5, 0.1, 1.0];
        const COLOR: [f32; 4] = [0.0, 0.9, 0.0, 1.0];

        let c_width = args.width as f64 / WIDTH as f64;
        let c_height = args.height as f64 / HEIGHT as f64;

        let live_req = rectangle::rectangle_by_corners(0.0, 0.0, c_width, c_height);
        let field = self.field;
        self.gl.draw(args.viewport(), |c, gl| {
            clear(BACKGROUND, gl);
            for i in 0..HEIGHT {
                for j in 0..WIDTH {
                    if field[i * WIDTH + j] > 0 {
                        let transform = c.transform
                            .trans(c_width * (j as f64), c_height * (i as f64));
                        rectangle(COLOR, live_req, transform, gl);
                    }
                }
            }
        });
    }

    fn at(&self, i: usize, j: usize) -> i32 {
        self.field[i * WIDTH + j]
    }

    fn calc_around(&self, i: usize, j: usize) -> i32 {
        let mut result = 0;
        if i > 0 {
            result += self.at(i - 1, j);
            if j > 0 {
                result += self.at(i - 1, j - 1);
            }
            if j < WIDTH - 1 {
                result += self.at(i - 1, j + 1);
            }
        }
        if j > 0 {
            result += self.at(i, j - 1);
        }
        if j < WIDTH - 1 {
            result += self.at(i, j + 1);
        }
        if i < HEIGHT - 1 {
            result += self.at(i + 1, j);
            if j > 0 {
                result += self.at(i + 1, j - 1);
            }
            if j < WIDTH - 1 {
                result += self.at(i + 1, j + 1);
            }
        }
        result
    }

    fn update(&mut self, args: &UpdateArgs) {
        if !self.run {
            return;
        }
        const RATE: f64 = 60.0;
        self.time += args.dt;
        let full_sec = (self.time * RATE) as u32;
        if full_sec == 0 {
            return;
        }
        for _ in 0..full_sec {
            let mut new_field = [0; WIDTH * HEIGHT];
            for i in 0..HEIGHT {
                for j in 0..WIDTH {
                    let around = self.calc_around(i, j);
                    new_field[i * WIDTH + j] = if self.at(i, j) > 0 {
                        if around < 2 || around > 3 {
                            0
                        } else {
                            1
                        }
                    } else {
                        around == 3
                        // if around == 3 {
                        //     1
                        // } else {
                        //     0
                        // }
                    }
                }
            }
            self.field = new_field;
        }
        self.time -= full_sec as f64 / RATE;
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window =
        WindowSettings::new("Conway's (?) Live", [WIDTH as u32, HEIGHT as u32])
            .opengl(opengl)
            .exit_on_esc(true)
            .build()
            .unwrap();

    let mut rnd = rand::thread_rng();
    let mut game_field: [i32; WIDTH * HEIGHT] = [0; WIDTH * HEIGHT];
    for i in 0..game_field.len() {
        game_field[i] = rnd.gen_range(0, 2);
    }
    let mut app = App {
        gl: GlGraphics::new(opengl),
        run: false,
        field: game_field,
        time: 0.0,
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(Button::Mouse(_button)) = e.press_args() {
            app.run = true;
        }

        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}
