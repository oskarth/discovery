#![deny(unsafe_code)]
#![no_main]
#![no_std]

extern crate panic_semihosting;

//use aux5::{entry, prelude::*, Delay, Leds};
use cortex_m_semihosting::dbg;

// B1 PA0 for button
// PE8 for blue led

#[cortex_m_rt::entry]
fn main() -> ! {
    //let (mut delay, mut leds): (Delay, Leds) = aux5::init();
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32f3::stm32f303::Peripherals::take().unwrap();

    // 2. enable GPIOA and SYSCFG clocks
    let rcc = &dp.RCC;
    rcc.ahbenr.modify(|_, w| {
        w.iopaen().set_bit()
            .iopeen().set_bit()
    });
    rcc.apb2enr.modify(|_, w| w.syscfgen().set_bit());

    //let half_period = 500_u16;

    dbg!("Hello world");

    loop {}
    // loop {
    //     leds[0].on();
    //     delay.delay_ms(half_period);

    //     leds[0].off();
    //     delay.delay_ms(half_period);
    // }
}
