use panic_halt as _;

pub struct FloatData {
    pub tmp: f32,
    pub hum: f32,
}

impl FloatData {
    pub fn calc_thi(&self) -> u32 {
        (0.81 * self.tmp + 0.01 * self.hum * (0.99 * self.tmp - 14.3) + 46.3) as u32
    }
    pub fn set(&mut self, tmp: f32, hum: f32) {
        self.tmp = tmp;
        self.hum = hum;
    }
}
