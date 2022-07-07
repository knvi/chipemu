use std::time;

pub struct Timer {
    delay_timer: u8,
    timer_set_time: time::Instant,
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            delay_timer: 0,
            timer_set_time: time::Instant::now(),
        }
    }

    pub fn set_timer(&mut self, value: u8) {
        self.delay_timer = value;
        self.timer_set_time = time::Instant::now();
    }

    pub fn get_timer(&self) -> u8 {
        let diff = time::Instant::now() - self.timer_set_time;
        let ms = diff.subsec_millis() as u64;
        if ms / 16 >= self.delay_timer as u64 {
            0
        } else {
            self.delay_timer - (ms / 16) as u8
        }
    }
}
