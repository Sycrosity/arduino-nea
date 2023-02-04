#![no_std]
#![no_main]
#![allow(unused_variables, unused_mut)]

use arduino_hal::{prelude::*, simple_pwm::*, Peripherals, Pins};

use arduino_hal::simple_pwm as pwm;

// use panic_halt as _;
use arduino_nea::prelude::*;

#[arduino_hal::entry]
fn main() -> ! {
    let dp: Peripherals = arduino_hal::Peripherals::take().unwrap();
    let pins: Pins = arduino_hal::pins!(dp);

    //[SETUP] setup vars
    let timer1 = pwm::Timer1Pwm::new(dp.TC1, pwm::Prescaler::Prescale1024);

    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    //bluetooth EN pin for enabling AT command mode (ON = AT mode)
    let mut blue_en = pins.d4.into_output();
    let mut laser = pins.d7.into_output();

    //must be a PWM~ pin
    let x_servo = pins.d9.into_output().into_pwm(&timer1);
    let mut x_servo = ServoUnit::new(x_servo, 8, 40);
    x_servo.calibrate_servo();

    //must be a PWM~ pin
    let y_servo = pins.d10.into_output().into_pwm(&timer1);
    let mut y_servo = ServoUnit::new(y_servo, 5, 38);
    y_servo.calibrate_servo();

    //[SETUP] setup function (vars created here will be cleaned)
    {
        if BLUE_AT {
            blue_en.set_high();
        }
    }

    ufmt::uwriteln!(&mut serial, "Hello from Arduino!\r").void_unwrap();

    let mut led = pins.d13.into_output();

    #[allow(clippy::empty_loop)]
    loop {}
}
