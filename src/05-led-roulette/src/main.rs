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
    // RCC AHB bus, enable GPIO A and E
    rcc.ahbenr.modify(|_, w| {
        w.iopaen().set_bit()
            .iopeen().set_bit()
    });
    rcc.apb2enr.modify(|_, w| w.syscfgen().set_bit());

    let gpioa = &dp.GPIOA;
    unsafe { gpioa.moder.modify(|_, w| w.moder0().bits(0x00)); // input
             gpioa.pupdr.modify(|_, w| w.pupdr0().bits(0x10))
    }; // PD
    //gpioa.odr.modify(|_, w| w.odr0().set_bit());

    let gpioe = &dp.GPIOE;
    gpioe.moder.modify(|_, w| w.moder8().output()); // bits(0x01)
    gpioe.odr.modify(|_, w| w.odr8().set_bit());

    //let half_period = 500_u16;

    dbg!("Hello world");

    loop {
        dbg!(gpioa.idr.read().idr0().bit_is_set());
    }

    // loop {
    //     leds[0].on();
    //     delay.delay_ms(half_period);

    //     leds[0].off();
    //     delay.delay_ms(half_period);
    // }
}
