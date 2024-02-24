use panic_halt as _;

/// 取得した温度と湿度の構造体
///
/// # メンバー変数
///
/// * `tmp` - 温度
/// * `hum` - 相対湿度
pub struct FloatData {
    pub tmp: f32,
    pub hum: f32,
}


// 計算関連の実装
impl FloatData {
    /// 不快指数の計算
    pub fn calc_thi(&self) -> u32 {
        (0.81 * self.tmp + 0.01 * self.hum * (0.99 * self.tmp - 14.3) + 46.3) as u32
    }
    /// 温度と相対湿度を設定
    ///
    /// # 引数
    ///
    /// * `tmp` - 温度
    /// * `hum` - 相対湿度
    pub fn set(&mut self, tmp: f32, hum: f32) {
        self.tmp = tmp;
        self.hum = hum;
    }
}
