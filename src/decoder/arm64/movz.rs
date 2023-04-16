use {crate::decoder::register::Register, std::fmt::Formatter};

pub struct MOVZ
{
    is_64bit: bool,
    target: Register,
    immediate: u16,
    shift: u8,
}

// https://developer.arm.com/documentation/ddi0602/2023-03/Base-Instructions/MOVZ--Move-wide-with-zero-?lang=en

impl From<u32> for MOVZ
{
    fn from(instruction: u32) -> Self
    {
        assert_eq!(instruction & 0x52800000, 0x52800000);

        let sf = (instruction & 0x80000000) != 0;
        let imm16 = (instruction & 0x1FFFE0) >> 5;
        let rd = instruction & 0x1F;
        let hw = (instruction & 0x600000) >> 21;

        let register: Register = if ((hw & 0b10) == 0) && (sf == false)
        {
            rd.into()
        }
        else
        {
            (rd as u64).into()
        };

        Self {
            is_64bit: sf,
            immediate: imm16 as u16,
            target: register,
            shift: hw as u8,
        }
    }
}

impl std::fmt::Display for MOVZ
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "movz {}, {}", self.target, self.immediate)?;

        if self.shift != 0
        {
            panic!("Check this.");
            write!(f, "LSL #{:#X}", self.shift)?;
        }

        Ok(())
    }
}
