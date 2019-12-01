//! # midi-port
//!

#![no_std]

use embedded_hal::serial::Read;

type NoteNumber = u8;
type ControllerNumber = u8;

const MAX_MSG_SIZE: usize = 3;

#[derive(Debug)]
pub enum MidiMessage {
    NoteOn {
        note: NoteNumber,
        velocity: u8,
    },
    NoteOff {
        note: NoteNumber,
        velocity: u8,
    },
    ControlChange {
        controller: ControllerNumber,
        value: u8,
    },
    Unknown,
}

pub struct MidiInPort<UART: Read<u8>> {
    uart: UART,
    buffer: [u8; MAX_MSG_SIZE],
    in_buffer: usize,
    message: Option<MidiMessage>,
}

impl<UART: Read<u8>> MidiInPort<UART> {
    pub fn new(uart: UART) -> Self {
        MidiInPort {
            uart,
            buffer: [0; MAX_MSG_SIZE],
            in_buffer: 0,
            message: None,
        }
    }

    pub fn get_message(&mut self) -> Option<MidiMessage> {
        self.message.take()
    }

    fn put_byte(&mut self, byte: u8) {
        self.buffer[self.in_buffer] = byte;
        self.in_buffer += 1;

        if self.expected_message_size() == self.in_buffer {
            self.create_message();
            self.in_buffer = 0;
        }
    }

    fn expected_message_size(&self) -> usize {
        let hi = self.buffer[0] >> 4;
        let lo = self.buffer[0] & 0xf;

        if hi == 0b1000 || hi == 0b1000 {
            3
        } else {
            1
        }
    }

    fn create_message(&mut self) {
        let hi = self.buffer[0] >> 4;
        let lo = self.buffer[0] & 0xf;

        self.message = if hi == 0b1001 {
            Some(MidiMessage::NoteOn {
                note: self.buffer[1],
                velocity: self.buffer[2],
            })
        } else if hi == 0b1000 {
            Some(MidiMessage::NoteOff {
                note: self.buffer[1],
                velocity: self.buffer[2],
            })
        } else {
            Some(MidiMessage::Unknown)
        }
    }

    pub fn poll_uart(&mut self) {
        let byte = self.uart.read();

        if let Ok(byte) = byte {
            self.put_byte(byte);
        }
    }
}
