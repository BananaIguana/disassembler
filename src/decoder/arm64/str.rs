use {crate::decoder::register::Register, std::fmt::Formatter};

pub struct STR
{
    is_64bit: bool,
    target: Register,
    source: Register,
}

impl From<u32> for STR
{
    fn from(instruction: u32) -> Self
    {
        assert_eq!(instruction & 0x80000000, 0x80000000);

        let size = (instruction & 0x40000000) >> 30;
        let is_64bit = size != 0;

        let rt = instruction & 0x1F;
        let rn = (instruction & 0x3E0) >> 5;
        let s = (instruction & 0x1000) >> 12;
        let option = (instruction & 0xE000) >> 13;
        let rm = (instruction & 0x1F0000) >> 16;

        let rt: Register = if is_64bit { (rt as u64).into() } else { rt.into() };
        let rn: Register = (rn as u64).into();
        let rm: Register = if (option & 0x4) != 0 { (rm as u64).into() } else { rm.into() };

        let shift = if is_64bit { s * 2 } else { s * 3 };

        Self {
            is_64bit,
            target: rt,
            source: rn,
        }
    }
}

impl std::fmt::Display for STR
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "str {}, [{}, #{:#X}]", self.target, self.source, 0)?;

        Ok(())
    }
}
