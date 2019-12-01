#![no_main]
#![no_std]

extern crate panic_semihosting;

use cortex_m::peripheral::Peripherals;
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

use hal::gpio::*;
use hal::rcc::RccExt;
use hal::serial::config::*;
use hal::serial::*;
use hal::stm32;
use stm32f4xx_hal as hal;

use midi_port::*;

#[entry]
fn main() -> ! {
    if let (Some(p), Some(_cp)) = (stm32::Peripherals::take(), Peripherals::take()) {
        let rcc = p.RCC.constrain();

        let clocks = rcc
            .cfgr
            .sysclk(stm32f4xx_hal::time::MegaHertz(168))
            .freeze();

        let gpioc = p.GPIOC.split();

        let rx_pin = gpioc.pc11.into_alternate_af8();

        let uart = Serial::uart4(
            p.UART4,
            (NoTx, rx_pin),
            Config {
                baudrate: stm32f4xx_hal::time::Bps(31250),
                wordlength: WordLength::DataBits8,
                parity: Parity::ParityNone,
                stopbits: StopBits::STOP1,
            },
            clocks,
        )
        .unwrap();

        let mut midi_in = MidiInPort::new(uart);

        loop {
            midi_in.poll_uart();

            if let Some(message) = midi_in.get_message() {
                hprintln!("{:?}", message).unwrap();
            }
        }

        // loop {
        //     match (uart.read()) {
        //         Ok(byte) => hprintln!("{:x}", byte).unwrap(),
        //         Err(e) => (),
        //     };
        // }
    }

    panic!();
}
