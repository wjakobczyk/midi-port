#![no_main]
#![no_std]

extern crate panic_semihosting;

use cortex_m::peripheral::Peripherals;
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

use stm32f4::stm32f407::interrupt;

use hal::gpio::*;
use hal::rcc::RccExt;
use hal::serial::config::*;
use hal::serial::*;
use hal::stm32;
use hal::stm32::UART4;
use stm32f4xx_hal as hal;

use midi_port::*;

type MidiUart = Serial<UART4, (NoTx, gpioc::PC11<Alternate<AF8>>)>;
static mut MIDI: *mut MidiInPort<MidiUart> = 0 as *mut MidiInPort<MidiUart>;

#[entry]
fn main() -> ! {
    if let (Some(p), Some(mut _cp)) = (stm32::Peripherals::take(), Peripherals::take()) {
        let rcc = p.RCC.constrain();

        let clocks = rcc
            .cfgr
            .sysclk(stm32f4xx_hal::time::MegaHertz(168))
            .freeze();

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
        )
        .unwrap();

        uart.listen(Event::Rxne);
        unsafe {
            cortex_m::peripheral::NVIC::unmask(stm32f4::stm32f407::Interrupt::UART4);
        }

        let mut midi_in = MidiInPort::new(uart);

        unsafe {
            MIDI = &mut midi_in;
        }

        loop {}
    }

    panic!();
}

#[interrupt]
fn UART4() {
    unsafe {
        (*MIDI).poll_uart();

        if let Some(message) = (*MIDI).get_message() {
            //the hprintln is taking long time so a lot of messages are lost in the meantime
            hprintln!("{:?}", message).unwrap();
        }
    }
}
