use {
    crate::decoder::register::Register,
    register_bits::{self, prelude::*, reg32::Reg32Bits},
    std::fmt::Formatter,
};

/// Form PC-relative address adds an immediate value to the PC value to form a PC-relative
/// address, and writes the result to the destination register.

#[allow(clippy::upper_case_acronyms)]
pub struct ADR
{
    register: Register,
    label: u64,
}

impl From<u32> for ADR
{
    fn from(instruction: u32) -> Self
    {
        debug_assert_eq!((instruction & 0x1F000000) >> 24, 0b10000);

        // 2 bits
        let immlo = (instruction & 0x60000000) >> 29;
        // 19 bits
        let immhi = (instruction & 0xFFFFE0) >> 5;
        let rd = instruction & 0x1F;

        debug_assert!(immhi <= 0x7FFFF);
        debug_assert!(immlo <= 0x3);

        let immediate = Reg32Bits::new(immhi << 2 | immhi);

        let immediate: Reg32Bits<21> = immediate.take_low();
        let sign_extended: Reg32Bits<32> = immediate.sign_extend();

        Self {
            register: rd.into(),
            label: u64::from(sign_extended),
        }
    }
}

impl std::fmt::Display for ADR
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "adr {}, {:#X}", self.register, self.label)
    }
}
