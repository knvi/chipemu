pub struct Chip8 {
    mem: [u8; 4096],
    vx: [u8; 16],
    regs: [u8; 16],
    pc: u16,
}

impl Chip8 {
    pub fn new() -> Self {
        let mut res = Chip8 {
            mem: [0; 4096],
            vx: [0; 16],
            regs: [0; 16],
            pc: 0x200,
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

    pub fn load_rom(&mut self, data: &Vec<u8>) {
        for i in 0..data.len() {
            self.mem[self.pc as usize + i] = data[i];
        }
    }

    pub fn decode(&mut self) {
        let hi = self.mem[self.pc as usize] as u16;
        let lo = self.mem[self.pc as usize + 1] as u16;
        let raw: u16 = (hi << 8) | lo;
        println!("Instruction read {:#X}: hi{:#X} lo:{:#X} ", raw, hi, lo);

        // variable declaration
        let nnn = raw & 0x0FFF; // 12 bit value
        let nn = (raw & 0xFF) as u8;
        let n = (raw & 0x00F) as u8;
        let x = ((raw & 0x0F00) >> 8) as u8;
        let y = ((raw & 0x00F0) >> 4) as u8;
        println!("nnn={:?}, nn={:?}, n={:?} x={}, y={}", nnn, nn, n, x, y);

        match raw {
            0x00E0 => println!("CLS"),
            0x00EE => println!("RETURN"),
            0x0000..=0x0FFF => {
                // 0nnn
                let addr = nnn;
                println!("SYS {addr}");
            }

            0x1000..=0x1FFF => {
                // 1nnn
                println!("JUMP {nnn}");
                self.pc = nnn;
            }

            0x3000..=0x3FFF => {
                // 3xkk
                let vx = self.vx[x as usize];
                if vx == nn {
                    self.pc += 2;
                }
            }

            0x4000..=0x4FFF => {
                // 4xkk
                let vx = self.vx[x as usize];
                if vx != nn {
                    self.pc += 2;
                }
            }

            0x5000..=0x5FF0 if n == 0 => {
                // 5xy0
                let vx = self.vx[x as usize];
                let vy = self.vx[y as usize];
                if vx == vy {
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
                    self.pc += 2;
                }
            }

            _ => todo!(), // unimplemented soft
        };
    }
}
