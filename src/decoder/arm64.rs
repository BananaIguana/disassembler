use {
    crate::{
        decoder::{Decoder, DecoderOutput},
        mode::Mode,
    },
    byteorder::LittleEndian,
};

pub struct Arm64Decoder {}

impl Arm64Decoder {}

impl Default for Arm64Decoder
{
    fn default() -> Self
    {
        Self {}
    }
}

impl Decoder for Arm64Decoder
{
    fn decode(&self, machine_code: &[u8], mode: Mode) -> DecoderOutput
    {
        // ARM64 instructions are always 4 bytes in size.
        let chunks = input.chunks(4);

        for (idx, mut chunk) in chunks.enumerate()
        {
            let instruction = chunk
                .read_u32::<LittleEndian>()
                .map_err(DecodingError::from)
                .map(|e| e)?;
        }

        DecoderOutput {
            info: vec![],
            result: vec![],
        }
    }
}
