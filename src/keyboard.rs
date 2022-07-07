pub struct Keyboard {
    key_pressed: u8,
}


impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard { key_pressed: 0 }
    }

    //Todo implement proper key handling
    pub fn is_key_pressed(&self, key_code: u8) -> bool {
        if let key = self.key_pressed {
            key == key_code
        } else {
            false
        }
    }

    pub fn set_key_pressed(&mut self, key: u8) {
        self.key_pressed = key;
    }

    pub fn get_key_pressed(&self) -> u8 {
        self.key_pressed
    }
}