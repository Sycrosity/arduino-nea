#![no_std]
#![no_main]

pub mod map_range;
pub mod panic;
pub mod servo;

//global CONSTS
pub const X_SERVO_CENTER: u8 = 17u8;
pub const X_SERVO_RIGHT: u8 = 34u8;
pub const X_SERVO_LEFT: u8 = 0u8;

pub const Y_SERVO_CENTER: u8 = 19u8;
pub const Y_SERVO_RIGHT: u8 = 38u8;
pub const Y_SERVO_LEFT: u8 = 0u8;

///whether the bluetooth module will be started in AT command mode.
pub const BLUE_AT: bool = true;

pub mod prelude {

    pub use super::{
        BLUE_AT, X_SERVO_CENTER, X_SERVO_LEFT, X_SERVO_RIGHT, Y_SERVO_CENTER, Y_SERVO_LEFT,
        Y_SERVO_RIGHT,
    };

    //imports
    pub use crate::map_range::*;
    pub use crate::panic as _;
    pub use crate::servo::*;
    pub use arduino_hal::delay_ms;
    pub use num_traits::ToPrimitive;
}
