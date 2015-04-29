use hack_interpreter::{Rom, Cpu};

use piston::window::{WindowSettings, Size};
use piston::input::Button;
use piston::input::keyboard::Key;
use piston::event::{PressEvent, ReleaseEvent, RenderEvent, UpdateEvent, Events};
use graphics;
use opengl_graphics::{GlGraphics, Texture, OpenGL};
use image::{Rgba, ImageBuffer, GenericImage};
use sdl2_window::Sdl2Window as Window;

use std::rc::Rc;
use std::cell::RefCell;

const WIDTH: usize = 512;
const HEIGHT: usize = 256;
const SCALE: usize = 2;
const SCREEN_ADDR: usize = 16384;
const SCREEN_MEMORY_LEN: usize = WIDTH * HEIGHT / 16;
const KEYBOARD_ADDR: usize = SCREEN_ADDR + SCREEN_MEMORY_LEN;

// TODO: MATH - Decide the MHz of the Cpu, partition it between frames
const CYCLES_PER_UPDATE: usize = 40000;
const MAX_FPS: u64 = 30;
const UPDATES_PER_SEC: u64 = 60;

pub fn run_simulator(input: &str) {
    let program = Rom::from_file(&input).unwrap();
    let ref mut cpu = Cpu::new(program);
    println!("Running program file: '{}'", input);

    let gl_version = OpenGL::_3_2;
    let window = Window::new(
        gl_version,
        WindowSettings::new(
            format!("hack-interpreter: {}", input),
            Size {
                width:  (WIDTH * SCALE) as u32,
                height: (HEIGHT * SCALE) as u32
            }
        ).exit_on_esc(true)
    );

    let window = Rc::new(RefCell::new(window));

    let ref mut image = ImageBuffer::new((WIDTH * SCALE) as u32, (HEIGHT * SCALE) as u32);
    let mut texture = Texture::from_image(&image);
    let ref mut gl = GlGraphics::new(gl_version);

    for e in window.events()
                   .ups(UPDATES_PER_SEC)
                   .max_fps(MAX_FPS) {
        if let Some(Button::Keyboard(key)) = e.press_args() {
            // HACK: Pong is expecting 'ASCII' keycodes of
            // 130 and 132 for left and right movement even
            // even though 130 and 132 are not actually ASCII.
            // Will probably need to remap a bunch of other keys.
            cpu.ram[KEYBOARD_ADDR] = match key {
                Key::Left => 130,
                Key::Right => 132,
                key => key as u16
            };
        }

        if let Some(Button::Keyboard(_)) = e.release_args() {
            cpu.ram[KEYBOARD_ADDR] = 0;
        }

        if let Some(args) = e.render_args() {
            gl.draw(
                args.viewport(),
                |c, g| graphics::image(&texture, c.transform, g)
            );
        }

        e.update(|_| {
            render_screen(image, &cpu);
            texture.update(image);
            for _ in (0..CYCLES_PER_UPDATE) { cpu.step() }
        });
    }
}

fn render_screen(image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, cpu: &Cpu) {
    let screen = &cpu.ram[SCREEN_ADDR..SCREEN_ADDR + SCREEN_MEMORY_LEN];

    for (idx, word) in screen.iter().enumerate() {
        // For each word of memory, draw 16 pixels
        let idx = idx * 16;
        for (bit, i) in (idx..idx + 16).enumerate() {
            let (x, y) = ((i % WIDTH) * SCALE, (i / WIDTH) * SCALE);
            let color = if word & (1 << bit) != 0 { 0 } else { 255 };

            for i in (0..SCALE) {
                for j in (0..SCALE) {
                    image.put_pixel((x + i) as u32,
                                    (y + j) as u32,
                                    Rgba([color, color, color, 255]));
                }
            }
        }
    }
}
