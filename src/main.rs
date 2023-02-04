#![no_std]
#![no_main]

use arduino_hal::{prelude::*, Peripherals, Pins};

use panic_halt as _;

//global CONSTS
///whether the bluetooth module will be in AT command mode
const BLUE_AT: bool = true;

#[arduino_hal::entry]
fn main() -> ! {
    let dp: Peripherals = arduino_hal::Peripherals::take().unwrap();
    let pins: Pins = arduino_hal::pins!(dp);

    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    //bluetooth EN pin for enabling AT command mode (ON = AT mode)
    let mut blue_en = pins.d4.into_output();

    //doesn't work due to RX/TX confusion requiring it to be on pins 0 and 1, so will just use the macro instead
    // // bluetooth RX pin
    // let blue_rx = pins.d2.into_floating_input();
    // //bluetooth TX pin
    // let blue_tx = pins.d3.into_output();

    //setup function
    {
        if BLUE_AT {
            blue_en.set_high()
        }
    }

    // // RX TO D2
    // // TX TO D3
    // // BAUD 57600
    // let blue_serial = arduino_hal::usart::Usart::new(dp.USART0, blue_rx, blue_tx, 57600.into());

    ufmt::uwriteln!(&mut serial, "Hello from Arduino!\r").void_unwrap();

    /*
     * For examples (and inspiration), head to
     *
     *     https://github.com/Rahix/avr-hal/tree/main/examples
     *
     * NOTE: Not all examples were ported to all boards!  There is a good chance though, that code
     * for a different board can be adapted for yours.  The Arduino Uno currently has the most
     * examples available.
     */

    let mut led = pins.d13.into_output();

    loop {
        // Read a byte from the serial connection
        let b = nb::block!(serial.read()).void_unwrap();

        // Answer
        ufmt::uwriteln!(&mut serial, "Got {}!\r", b).void_unwrap();
        led.toggle();
    }
}
