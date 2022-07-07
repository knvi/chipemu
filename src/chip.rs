use minifb::Key;
use rand::{self, distributions::{Range, IndependentSample}};
use crate::display::Display;
use crate::keyboard::Keyboard;
use crate::timer::Timer;

pub struct Chip8 {
    mem: [u8; 4096],
    vx: [u8; 16],
    stack: Vec<u16>,
    i: u16,
    pc: u16,
    screen: Display,
    keyboard: Keyboard,
    timer: Timer
}

impl Chip8 {
    pub fn new() -> Self {
        let mut res = Chip8 {
            mem: [0; 4096],
            vx: [0; 16],
            stack: Vec::new(),
            i: 0,
            pc: 0x200,
            screen: Display::new(),
            keyboard: Keyboard::new(),
            timer: Timer::new()
        };

        res.mem[0..16 * 5].copy_from_slice(&[
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80, // F
        ]);

        res
    }

    fn draw_sprite(&mut self, x: u8, y: u8, h: u8) {
        let mut should_set_vf = false;
        for sprite_y in 0..h {
            let byte = self.mem[self.i as usize + sprite_y as usize];
            if self.screen.draw_byte(byte, x, y + sprite_y) {
                should_set_vf = true;
            }
        }
        if should_set_vf {
            self.vx[0xF] = 1;
        } else {
            self.vx[0xF] = 0;
        }
    }

    pub fn load_rom(&mut self, data: &Vec<u8>) {
        for i in 0..data.len() {
            self.mem[self.pc as usize + i] = data[i];
        }
    }

    pub fn get_display_buffer(&self) -> &[u8] {
        self.screen.get_dis_buf()
    }

    pub fn decode(&mut self) {
        let hi = self.mem[self.pc as usize] as u16;
        let lo = self.mem[self.pc as usize + 1] as u16;
        let raw: u16 = (hi << 8) | lo;

        // variable declaration
        let nnn = raw & 0x0FFF; // 12 bit value
        let nn = (raw & 0xFF) as u8;
        let n = (raw & 0x00F) as u8;
        let x = ((raw & 0x0F00) >> 8) as u8;
        let y = ((raw & 0x00F0) >> 4) as u8;

        match raw {
            0x0000..=0x0FFF => {
                match nn {
                    0xE0 => {
                        println!("SCREEN_CLEAR");
                        self.pc += 2;
                    }

                    0xEE => {
                        // ret from sub
                        let addr = self.stack.pop().unwrap();
                        self.pc = addr;
                    }

                    _ => panic!("invalid 0x instruction")
                }

                // 0nnn
                let addr = nnn;
                println!("SYS {addr}");
            }

            0x1000..=0x1FFF => {
               
                // 1nnn
                println!("JUMP {nnn}");
                self.pc = nnn;
            }

            0x2000..=0x2FFF => {
                self.stack.push(self.pc + 2);
                self.pc = nnn;
            }

            0x3000..=0x3FFF => {
                // 3xkk
                let vx = self.vx[x as usize];
                if vx == nn {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }

            0x4000..=0x4FFF => {
                // 4xkk
                let vx = self.vx[x as usize];
                if vx != nn {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }

            0x5000..=0x5FF0 if n == 0 => {
                // 5xy0
                let vx = self.vx[x as usize];
                let vy = self.vx[y as usize];
                if vx == vy {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }

            0x6000..=0x6FFF => {
                //6xkk
                self.vx[x as usize] = nn;
                self.pc +=2;
            }

            0x7000..=0x7FFF => {
                //7xkk
                self.vx[x as usize] += nn;
                self.pc +=2;
            }

            0x8000..=0x8FFF => { match n {
                0x0 => {
                    self.vx[x as usize] = self.vx[y as usize];
                }

                0x1 => {
                    self.vx[x as usize] = (self.vx[x as usize] | self.vx[y as usize]);
                }

                0x2 => {
                    self.vx[x as usize] = (self.vx[x as usize] & self.vx[y as usize]);
                }

                0x3 => {
                    self.vx[x as usize] = (self.vx[x as usize] ^ self.vx[y as usize]);
                }

                0x4 => {
                    let vx: u16 = self.vx[x as usize] as u16;
                    let vy: u16 = self.vx[y as usize] as u16;

                    let sum: u16 = vx + vy;
                    self.vx[x as usize] = sum as u8;
                    if sum > 0xFF {
                        self.vx[0xF] = 1;
                    }
                }

                0x5 => {
                    let vx: i8 = self.vx[x as usize] as i8;
                    let vy: i8 = self.vx[y as usize] as i8;

                    if vx > vy {
                        self.vx[0xF] = 1;
                    } else {
                        self.vx[0xF] = 0;
                    }
                    self.vx[x as usize] -= self.vx[y as usize];
                }

                0x6 => {
                    let val = self.vx[x as usize];
                    if (val & 0x1) == 0x1 {
                        self.vx[0xF] = 1;
                    } else {
                        self.vx[0xF] = 0;
                    }

                    self.vx[x as usize] = val >> 1;
                }

                0x7 => {
                    let diff: i8 = self.vx[y as usize] as i8 - self.vx[x as usize] as i8;
                    self.vx[x as usize] = diff as u8;
                    if diff < 0 {
                        self.vx[0xF] = 1;
                    } else {
                        self.vx[0xF] = 0;
                    }
                }

                0xE => {
                    let val = self.vx[x as usize];
                    if val & 0x80 == 0x80 {
                        self.vx[0xF] = 1;
                    } else {
                        self.vx[0xF] = 0;
                    }

                    self.vx[x as usize] = val << 1;
                }
                _ => panic!("error")
                
                }
                self.pc+=2;
            },

            0x9000..=0x9FF0 if n == 0 => { // 9xy0
                let vx = self.vx[x as usize];
                let vy = self.vx[y as usize];
                if vx != vy {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }

            0xA000..=0xAFFF => {    // Annn
                
                self.i = nnn;
                self.pc += 2;
            }

            0xB000..=0xBFFF => {    //Bnnn
                self.pc = self.vx[0] as u16 + nnn;
            }

            0xC000..=0xCFFF => {
                let mut rng = rand::thread_rng();
                let interval = Range::new(0, 255);
                let num = interval.ind_sample(&mut rng);
                self.vx[x as usize] = num;
                
                self.pc += 2;
            }

            0xD000..=0xDFFF => {
                let vx = self.vx[x as usize];
                let vy = self.vx[y as usize];
                self.draw_sprite(vx, vy, n);
                self.pc += 2;
            }

            0xE000..=0xEFFF => {
                match nn {
                    0xA1 => {
                        let key = self.vx[x as usize];
                        if !self.keyboard.is_key_pressed(key) {
                            self.pc += 4;
                        } else {
                            self.pc += 2;
                        }
                    }

                    0x9E => {
                        let key = self.vx[x as usize];
                        if self.keyboard.is_key_pressed(key) {
                            self.pc += 4;
                        } else {
                            self.pc += 2;
                        }
                    }

                    _ => {
                        panic!("Invalid 0xE exception!");
                    }
                }
            }

            0xF000..=0xFFFF => {
                match nn {
                    0x07 => {
                        self.vx[x as usize] = self.timer.get_timer();
                        self.pc += 2;
                    }
                    0x0A => {
                        if let val = self.keyboard.get_key_pressed() {
                            self.vx[x as usize] = val;
                            self.pc += 2 ;
                        }
                    }

                    0x15 => {
                        self.timer.set_timer(self.vx[x as usize]);
                        self.pc += 2;
                    }

                    0x18 => {
                        self.pc += 2; // i dont have sound yet and i dont think i will implement it
                    }

                    0x29 => {
                        self.i = self.vx[x as usize] as u16 * 5;
                        self.pc += 2;
                    }

                    0x1E => {
                        let vx = self.vx[x as usize];
                        self.i += vx as u16;
                        self.pc +=2;
                    }

                    0x33 => {
                        let vx = self.vx[x as usize];
                        self.mem[self.i as usize] = vx / 100;
                        self.mem[self.i as usize + 1] = (vx % 100) / 10;
                        self.mem[self.i as usize + 2] = vx % 10;
                        self.pc += 2;
                    }

                    0x55 => {
                        for index in 0..x + 1 {
                            let v = self.vx[index as usize];
                            self.mem[self.i as usize + index as usize] = v;
                        }
                        self.i += x as u16 + 1;
                        self.pc +=2;
                    }

                    0x65 => {
                        for index in 0..x + 1 {
                            let v = self.mem[self.i as usize + index as usize];
                            self.vx[index as usize] = v;
                        }
                        self.i += x as u16 + 1;
                        self.pc +=2;
                    }
                    _ => panic!("not implemented Fx soething")
                }
            }

            _ => todo!(), // unimplemented soft
        };
    }

    pub fn set_key_pressed(&mut self, key: u8 ) {
        self.keyboard.set_key_pressed(key);
    }
}
