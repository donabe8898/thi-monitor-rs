pub trait Device {
    fn calc_thi(&self) -> u32 {
        0
    }
    fn set(&mut self, _tmp: f32, _hum: f32) {}
}
