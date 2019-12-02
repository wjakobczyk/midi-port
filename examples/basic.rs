#![no_main]
#![no_std]

extern crate panic_semihosting;

use cortex_m::peripheral::Peripherals;
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;


use hal::delay::Delay;
use hal::gpio::*;
use hal::rcc::RccExt;
use hal::stm32;
use hal::serial::*;
use hal::serial::config::*;
use stm32f4xx_hal as hal;

use embedded_hal::serial::Read;

#[entry]
fn main() -> ! {
    if let (Some(p), Some(cp)) = (stm32::Peripherals::take(), Peripherals::take()) {
        let rcc = p.RCC.constrain();

        let clocks = rcc
            .cfgr
            .sysclk(stm32f4xx_hal::time::MegaHertz(168))
            .freeze();

        let mut delay = Delay::new(cp.SYST, clocks);

        let gpioc = p.GPIOC.split();

        let rx_pin = gpioc.pc11.into_alternate_af8();

        let mut uart = Serial::uart4(
            p.UART4,
            (NoTx, rx_pin),
            Config {
                baudrate: stm32f4xx_hal::time::Bps(31250),
                wordlength: WordLength::DataBits8,
                parity: Parity::ParityNone,
                stopbits: StopBits::STOP1,
            },
            clocks,
        ).unwrap();

        loop {
            match (uart.read()) {
                Ok(byte) => hprintln!("{:x}", byte).unwrap(),
                Err(e) => (),
            };
            
        }
    }

    panic!();
}
