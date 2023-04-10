mod arm64;
mod instruction;
mod register;

use {
    crate::{arch::Architecture, decoder::arm64::Arm64Decoder, mode::Mode, report::Report},
    std::fmt::Formatter,
};

pub struct DecoderOutput
{
    result: Vec<String>,
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

impl std::fmt::Debug for DecoderOutput
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
    {
        for assembly_line in self.result.iter()
        {
            writeln!(f, "{}", assembly_line)?;
        }

        Ok(())
    }
}
