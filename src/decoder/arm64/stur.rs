use {
    crate::decoder::register::Register,
    register_bits::prelude::{Reg32Bits, Reg32BitsDownCast, Reg32BitsUpCast},
    std::fmt::Formatter,
};

pub struct STUR
{
    is_64bit: bool,
    target: Register,
    source: Register,
    offset: i32,
}

impl From<u32> for STUR
{
    fn from(instruction: u32) -> Self
    {
        let size = (instruction & 0x40000000) >> 30;

        let imm9 = (instruction & 0x1FF000) >> 12;
        let rn = (instruction & 0x3E0) >> 5;
        let rt = instruction & 0x1F;

        let is_64bit = size != 0;

        let rt: Register = if is_64bit { (rt as u64).into() } else { rt.into() };
        let immediate = Reg32Bits::new(imm9);

        let immediate: Reg32Bits<9> = immediate.take_low();
        let sign_extended: Reg32Bits<32> = immediate.sign_extend();
        let offset = u32::from(sign_extended);

        Self {
            is_64bit,
            offset: offset as i32,
            target: rt,
            source: (rn as u64).into(),
        }
    }
}

impl std::fmt::Display for STUR
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
    {
        write!(
            f,
            "stur {}, [{}, #{:#X}]",
            self.target, self.source, self.offset
        )?;

        Ok(())
    }
}
