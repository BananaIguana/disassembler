mod arm64;

use crate::{arch::Architecture, decoder::arm64::Arm64Decoder, mode::Mode};

pub struct Report
{
    byte_offset: usize,        // to the report
    instruction_offset: usize, // ""
    error: bool,
    description: String,
}

pub struct DecoderOutput
{
    result: Vec<String>, // Change to enum
    info: Vec<Report>,
}

pub trait Decoder
{
    fn decode(&self, machine_code: &[u8], mode: Mode) -> DecoderOutput;
}

pub fn decoder(arch: Architecture) -> impl Decoder
{
    match arch
    {
        Architecture::Arm64 => Arm64Decoder::default(),
    }
}
