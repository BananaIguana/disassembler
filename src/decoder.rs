mod arm64;

use crate::{arch::Architecture, decoder::arm64::Arm64Decoder, mode::Mode, report::Report};

pub struct DecoderOutput
{
    _result: Vec<String>,
}

pub trait Decoder
{
    fn decoder(mode: Mode) -> Self;
    fn decode(&mut self, machine_code: &[u8]) -> Result<DecoderOutput, Report>;
}

pub fn decoder(arch: Architecture, mode: Mode) -> impl Decoder
{
    match arch
    {
        Architecture::Arm64 => Arm64Decoder::decoder(mode),
    }
}
