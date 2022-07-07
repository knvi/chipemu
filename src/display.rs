pub struct Display {
    buf: [u8; 64*32]
}

impl Display {
    pub fn new() -> Display {
        Display {
            buf: [0; 64*32]
        }
    }

    pub fn get_ind_from_pos(x: usize, y: usize) -> usize {
        y * 64 + x
    }

    pub fn draw_byte(&mut self, byte: u8, x: u8, y: u8) -> bool {
        let mut erased = false;
        let mut coord_x = x as usize;
        let mut coord_y = y as usize;
        let mut b = byte;
        
        for _ in 0..8 {
            coord_x %= 64;
            coord_y %= 32;
            let index = Display::get_ind_from_pos(coord_x, coord_y);
            let bit = (b & 0b1000_0000) >> 7;
            let prev_value = self.buf[index];
            self.buf[index] ^= bit;

            if prev_value == 1 && self.buf[index] == 0 {
                erased = true;
            }

            coord_x += 1;
            b <<= 1;
        }

        erased
    }

    pub fn get_dis_buf(&self) -> &[u8] {
        &self.buf
    }
}