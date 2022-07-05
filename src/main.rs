use chip::Chip8;
use std::{fs::File, io::Read};

mod chip;

fn main() {
    println!("Hello, world!");
    let mut file = File::open("data/roms/INVADERS").unwrap();
    let mut data = Vec::<u8>::new();
    file.read_to_end(&mut data).unwrap();

    let mut chip = Chip8::new();
    chip.load_rom(&data);
    loop {
        chip.decode();
    }
}