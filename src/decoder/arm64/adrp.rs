use {
    crate::decoder::register::Register,
    register_bits::{self, prelude::*, reg64::Reg64Bits},
    std::fmt::Formatter,
};

/// Form PC-relative address to 4KB page adds an immediate value that is shifted left by 12 bits, to
/// the PC value to form a PC-relative address, with the bottom 12 bits masked out, and writes the
/// result to the destination register.
/// https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/ADRP--Form-PC-relative-address-to-4KB-page-?lang=en

#[allow(clippy::upper_case_acronyms)]
pub struct ADRP
{
    register: Register,
    label: u64, // +/- 4GB
}

impl From<u32> for ADRP
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

        let immediate = immhi << 2 | immhi;
        let immediate = Reg64Bits::new((immediate as u64) << 12);

        let immediate: Reg64Bits<33> = immediate.take_low();
        let sign_extended: Reg64Bits<64> = immediate.sign_extend();

        Self {
            register: rd.into(),
            label: u64::from(sign_extended),
        }
    }
}

impl std::fmt::Display for ADRP
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "adrp {}, {:#X}", self.register, self.label)
    }
}
