use arduino_hal::{
    hal::port::Dynamic,
    port::{mode::PwmOutput, Pin},
    simple_pwm::PwmPinOps,
};
use num_traits::ToPrimitive;

use crate::prelude::*;

///A high-level abstraction over a [Pin] with mode [PwmOutput], allowing for simplifed servo control, iteration (in future) and calibration.
pub struct ServoUnit<TC, PIN = Dynamic> {
    pub servo: Pin<PwmOutput<TC>, PIN>,
    ///don't just make 0! leave some wiggle room for the servo rotating too quickly
    pub min_duty: u8,
    pub max_duty: u8,
}

impl<TC, PIN: PwmPinOps<TC>> ServoUnit<TC, PIN> {
    pub fn new(servo: Pin<PwmOutput<TC>, PIN>, min_duty: u8, max_duty: u8) -> Self {
        Self {
            servo,
            min_duty,
            max_duty,
        }
    }

    pub fn calibrate_servo(&mut self) {
        self.servo.enable();
        self.servo.set_duty(self.min_duty);
        delay_ms(1000);
        self.servo.set_duty(self.max_duty / 2);
        delay_ms(1000);
        self.servo.set_duty(self.max_duty);
        delay_ms(1000);
        // self.servo.disable();
    }

    pub fn enable(&mut self) {
        self.servo.enable()
    }

    pub fn disable(&mut self) {
        self.servo.disable()
    }

    fn set_duty(&mut self, duty: u8) {
        self.servo.set_duty(duty);
    }

    pub fn set_angle(&mut self, angle: u8) {
        // let angle = angle
        //     .to_u16()
        //     //converting to u16 will be safe, as 255 * 255 (65_025, the max calculation that can be done with originally u8's) fits within the range of 2^16 (65_535).
        //     .unwrap()
        //     .map_range(0..180, self.min_duty.to_u16().unwrap()..self.max_duty.to_u16().unwrap());

        self.set_duty(
            angle
                .to_u16()
                //converting to u16 will be safe, as 255 * 255 (65_025, the max calculation that can be done with originally u8's) fits within the range of 2^16 (65_535).
                .unwrap()
                .map_range(
                    0..180,
                    self.min_duty.to_u16().unwrap()..self.max_duty.to_u16().unwrap(),
                )
                //convert back from a (safely in range) u16 to a u8 duty again
                .to_u8()
                .unwrap(),
        );
    }

    pub fn get_duty(&self) -> <PIN as PwmPinOps<TC>>::Duty {
        self.servo.get_duty()
    }

    // pub fn get_angle(&self) -> u8 {

    //     self.servo.get_duty()
    //     .to_u16()
    //     //converting to u16 will be safe, as 255 * 255 (65_025, the max calculation that can be done with originally u8's) fits within the range of 2^16 (65_535).
    //     .unwrap()
    //     .map_range(
    //         self.min_duty.to_u16().unwrap()..self.max_duty.to_u16().unwrap(),
    //         0..180,
    //     )
    //     //convert back from a (safely in range) u16 to a u8 duty again
    //     .to_u8()
    //     .unwrap()
    // }
}
