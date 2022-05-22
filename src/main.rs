// main.rs
//
#![no_std]
#![no_main]

extern crate panic_halt;
use arduino_uno::hal::port::mode::Output;
use arduino_uno::hal::port::portb::PB5;
use arduino_uno::prelude::*;

#[arduino_uno::entry]
fn main() -> ! {
    let mut switch_state = 0;
    let peripherals = arduino_uno::Peripherals::take().unwrap();

    let mut pins = arduino_uno::Pins::new(peripherals.PORTB, peripherals.PORTC, peripherals.PORTD);

    let button = pins.d2.into_floating_input(&mut pins.ddr);
    let mut led_green = pins.d3.into_output(&mut pins.ddr);
    let mut led_red = pins.d4.into_output(&mut pins.ddr);
    let mut led_red_sec = pins.d5.into_output(&mut pins.ddr);

    loop {
        if button.is_low().unwrap() {
            led_green.set_high();
            led_red.set_low();
            led_red_sec.set_low();
        } else {
            led_green.set_low();
            led_red.set_high();
            led_red_sec.set_high();

            arduino_uno::delay_ms(250 as u16);
            led_red.set_high();
            led_red_sec.set_low();
            arduino_uno::delay_ms(250 as u16);
        }
    }
}

fn stutter_blink(led: &mut PB5<Output>, times: usize) {
    (0..times).map(|i| i * 10).for_each(|i| {
        led.toggle().void_unwrap();
        arduino_uno::delay_ms(i as u16);
    });
}
