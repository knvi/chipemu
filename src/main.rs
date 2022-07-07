extern crate minifb;
extern crate rand;

use chip::Chip8;
use display::Display;
use minifb::{Key, KeyRepeat, Window, WindowOptions};
use std::{
    fs::File,
    io::Read,
    time::{Duration, Instant},
};
use timer::Timer;

mod chip;
mod display;
mod keyboard;
mod timer;

fn get_chip8_keycode_for(key: Key) -> u8 {
    match key {
        Key::Key1 => 0x1,
        Key::Key2 => 0x2,
        Key::Key3 => 0x3,
        Key::Key4 => 0xC,

        Key::Q => 0x4,
        Key::W => 0x5,
        Key::E => 0x6,
        Key::R => 0xD,

        Key::A => 0x7,
        Key::S => 0x8,
        Key::D => 0x9,
        Key::F => 0xE,

        Key::Z => 0xA,
        Key::X => 0x0,
        Key::C => 0xB,
        Key::V => 0xF,
        _ => 0,
    }
}

fn main() {
    let width = 640;
    let height = 320;

    let mut file = File::open("data/roms/INVADERS").unwrap();
    let mut data = Vec::<u8>::new();
    file.read_to_end(&mut data).expect("File not found!");
    let mut buffer: Vec<u32> = vec![0; width * height];

    let mut win = Window::new("chipemu", width, height, WindowOptions::default())
        .unwrap_or_else(|e| panic!("error creating window: {e}"));

    let mut chip8 = Chip8::new();
    chip8.load_rom(&data);

    let mut last_key_update_time = Instant::now();
    let mut last_instruction_run_time = Instant::now();
    let mut last_display_time = Instant::now();

    while win.is_open() && !win.is_key_down(Key::Escape) {
        let keys_pressed = win.get_keys_pressed(KeyRepeat::Yes);
        for key in keys_pressed {
            let keycode = get_chip8_keycode_for(key);
            if keycode != 0 || Instant::now() - last_key_update_time >= Duration::from_millis(200) {
                last_key_update_time = Instant::now();
                chip8.set_key_pressed(keycode);
            }
        }

        if Instant::now() - last_instruction_run_time > Duration::from_millis(2) {
            last_instruction_run_time = Instant::now();
            chip8.decode();
        }

        if Instant::now() - last_display_time > Duration::from_millis(10) {
            let display = chip8.get_display_buffer();

            for y in 0..height {
                let yc = y / 10;
                let off = yc * width;
                for x in 0..width {
                    let ind = Display::get_ind_from_pos(x / 10, yc);
                    let pixel = display[ind];

                    let color = match pixel {
                        0 => 0x0,
                        1 => 0xffffff,
                        _ => panic!("invalid pixel value"),
                    };
                    buffer[off + x] = color;
                }
            }
            win.update_with_buffer(&buffer, width, height).unwrap();
            last_display_time = Instant::now();
        }
    }
}
