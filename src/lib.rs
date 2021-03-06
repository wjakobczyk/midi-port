//! # midi-port
//!
//! This is a Rust driver library for UART midi port. 
//!
#![no_std]

use embedded_hal::serial::Read;

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

pub type ChannelNumber = u8;
pub type NoteNumber = u8;
pub type ControllerNumber = u8;
pub type ProgramNumber = u8;

const MAX_MSG_SIZE: usize = 3;

#[derive(Debug)]
pub enum MidiMessage {
    NoteOn {
        channel: ChannelNumber,
        note: NoteNumber,
        velocity: u8,
    },
    NoteOff {
        channel: ChannelNumber,
        note: NoteNumber,
        velocity: u8,
    },
    Aftertouch {
        channel: ChannelNumber,
        note: Option<NoteNumber>, //Null for channel aftertouch
        value: u8,
    },
    ControlChange {
        channel: ChannelNumber,
        controller: ControllerNumber,
        value: u8,
    },
    ProgramChange {
        channel: ChannelNumber,
        program: ProgramNumber,
    },
    PitchBendChange {
        channel: ChannelNumber,
        value: u16, //14 bits used
    },
    Unknown,
}

#[derive(FromPrimitive)]
enum Status {
    NoteOff = 0x80,
    NoteOn = 0x90,
    PolyphonicAftertouch = 0xA0,
    ControlChange = 0xB0,
    ProgramChange = 0xC0,
    ChannelAftertouch = 0xD0,
    PitchBend = 0xE0,
    SysExStart = 0xF0,
    TimeCodeQtrFrame = 0xF1,
    SongPositionPointer = 0xF2,
    SongSelect = 0xF3,
    TuneRequest = 0xF6,
    SysExEnd = 0xF7,
    TimingClock = 0xF8,
    Start = 0xFA,
    Continue = 0xFB,
    Stop = 0xFC,
    ActiveSensing = 0xFE,
    SystemReset = 0xFF,
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
        let hi = self.buffer[0] & 0xf0;

        match FromPrimitive::from_u8(hi) {
            Some(Status::NoteOn) => 3,
            Some(Status::NoteOff) => 3,
            Some(Status::PolyphonicAftertouch) => 3,
            Some(Status::ControlChange) => 3,
            Some(Status::ChannelAftertouch) => 2,
            Some(Status::PitchBend) => 3,
            _ => 1,
        }
    }

    fn create_message(&mut self) {
        let hi = self.buffer[0] & 0xf0;
        let lo = self.buffer[0] & 0xf;

        self.message = match FromPrimitive::from_u8(hi) {
            Some(Status::NoteOn) => Some(MidiMessage::NoteOn {
                channel: lo,
                note: self.buffer[1],
                velocity: self.buffer[2],
            }),
            Some(Status::NoteOff) => Some(MidiMessage::NoteOff {
                channel: lo,
                note: self.buffer[1],
                velocity: self.buffer[2],
            }),
            Some(Status::PolyphonicAftertouch) => Some(MidiMessage::Aftertouch {
                channel: lo,
                note: Some(self.buffer[1]),
                value: self.buffer[2],
            }),
            Some(Status::ControlChange) => Some(MidiMessage::ControlChange {
                channel: lo,
                controller: self.buffer[1],
                value: self.buffer[2],
            }),
            Some(Status::ChannelAftertouch) => Some(MidiMessage::Aftertouch {
                channel: lo,
                note: None,
                value: self.buffer[1],
            }),
            Some(Status::PitchBend) => Some(MidiMessage::PitchBendChange {
                channel: lo,
                value: self.buffer[1] as u16 + ((self.buffer[2] as u16) << 7),
            }),
            _ => Some(MidiMessage::Unknown),
        }
    }

    pub fn poll_uart(&mut self) {
        let byte = self.uart.read();

        if let Ok(byte) = byte {
            self.put_byte(byte);
        }
    }
}
