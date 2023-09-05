use {crate::decoder::register::Register, register_bits::prelude::*, std::fmt::Formatter};

/// Add (immediate) adds a register value and an optionally-shifted immediate value, and writes the
/// result to the destination register.
///
/// This instruction is used by the alias MOV (to/from SP).

#[allow(dead_code)]
pub enum Type
{
    PreIndex,
    PostIndex,
    SignedOffset,
}

#[allow(dead_code)]
#[allow(clippy::upper_case_acronyms)]
pub struct STP
{
    source1: Register,
    source2: Register,
    target: Register,
    immediate: i32,
    is_64bit: bool,
    stp_type: Type,
}

// https://developer.arm.com/documentation/ddi0596/2021-12/Base-Instructions/STP--Store-Pair-of-Registers-?lang=en

impl From<u32> for STP
{
    #[allow(clippy::if_same_then_else)]
    fn from(instruction: u32) -> Self
    {
        let stp_type = if (instruction & 0x28800000) == 0x28800000
        {
            panic!("Check this works!");
            //Type::PostIndex
        }
        else if (instruction & 0x29800000) == 0x29800000
        {
            panic!("Check this works!");
            //Type::PreIndex
        }
        else
        {
            assert_eq!(instruction & 0x29000000, 0x29000000);
            Type::SignedOffset
        };

        let opc = instruction & 0x80000000;
        let rt = (instruction & 0x1F) as u64;
        let rn = ((instruction & 0x3E0) >> 5) as u64;
        let rt2 = ((instruction & 0x7C00) >> 10) as u64;
        let imm = (instruction & 0x3F8000) >> 15;

        let is_64bit = opc != 0;

        let imm = Reg64Bits::new(imm as u64);
        let imm: Reg64Bits<7> = imm.take_low();
        let imm: Reg64Bits<64> = imm.sign_extend();
        let imm: u64 = u64::from(imm) << if is_64bit { 3 } else { 2 };

        let rn = Register::from(rn);
        let rt = Register::from(rt);
        let rt2 = Register::from(rt2);

        STP {
            source1: rt,
            source2: rt2,
            target: rn,
            immediate: imm as i32,
            is_64bit,
            stp_type,
        }
    }
}

impl std::fmt::Display for STP
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
    {
        write!(
            f,
            "stp {}, {}, [{}, #{:#X}]",
            self.source1, self.source2, self.target, self.immediate
        )?;

        Ok(())
    }
}
