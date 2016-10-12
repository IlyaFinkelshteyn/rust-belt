//! Defines the game component.

use piston_window::{Button, clear, Key, PressEvent, PistonWindow, polygon, Transformed, types};

use color;

const SHIP_HEIGHT: f64 = 16.0;
const SHIP_WIDTH: f64 = 20.0;

const SHIP: &'static types::Triangle = &[
    [0.0, -1.0 * SHIP_HEIGHT / 2.0],
    [SHIP_WIDTH, 0.0],
    [0.0, SHIP_HEIGHT / 2.0]
];

#[derive(Clone, Default)]
pub struct Size {
    pub width: f64,
    pub height: f64
}

pub struct Position {
    x: f64,
    y: f64,
}

/// Stores Game state.
pub struct Game {
    position: Position,
    rotation: f64,
}

impl Game {
    pub fn new() -> Self {
        Game {
            position: Position {
                x: 0.0,
                y: 0.0,
            },
            rotation: 0.0,
        }
    }

    pub fn run(&mut self, window: &mut PistonWindow) {
        while let Some(event) = window.next() {
            window.draw_2d(&event,
                           |context, graphics| {
                               clear(color::BLACK, graphics);
                               polygon(color::CYAN,
                                       SHIP,
                                       context.transform
                                           .trans(self.position.x,
                                                  self.position.y)
                                           .rot_rad(self.rotation)
                                           // Without this trans(), rotation occurs around the
                                           // upper left corner rather than the center.
                                           .trans(-1.0 * SHIP_HEIGHT / 2.0, 0.0),
                                       graphics);
                           });
            if let Some(button) = event.press_args() {
                match button {
                    Button::Keyboard(Key::D) => { self.position.x += 1.0 }
                    Button::Keyboard(Key::A) => { self.position.x -= 1.0 }
                    Button::Keyboard(Key::S) => { self.position.y += 1.0 }
                    Button::Keyboard(Key::W) => { self.position.y -= 1.0 }
                    Button::Keyboard(Key::Q) => { self.rotation -= 0.1 }
                    Button::Keyboard(Key::E) => { self.rotation += 0.1 }
                    Button::Keyboard(Key::X) => { break }
                    _ => {}
                }
            }
        }
    }
}