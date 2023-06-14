#![no_main]
#![no_std]

use arduino_hal::prelude::*;
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    let mut i2c = arduino_hal::I2c::new(
        dp.TWI,
        pins.a4.into_pull_up_input(),
        pins.a5.into_pull_up_input(),
        50000,
    );
    let dat = [0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8];
    let trigger = [0xACu8, 0x33u8, 0x00u8];
    arduino_hal::delay_ms(1000);

    ufmt::uwriteln!(&mut serial, "Write direction test:\r").void_unwrap();
    i2c.i2cdetect(
        &mut serial,
        arduino_hal::i2c::I2cOps::raw_write(&mut serial, 0xACu8),
    );
    ufmt::uwriteln!(&mut serial, "\r\nRead direction test\r").void_unwrap();
    i2c.i2cdetect(&mut serial, arduino_hal::i2c::Direction::Read);
    // let mut led = pins.d13.into_output();

    loop {
        arduino_hal::delay_ms(1000);
    }
}
