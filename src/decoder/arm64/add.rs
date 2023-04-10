use {crate::decoder::register::Register, std::fmt::Formatter};

/// Add (immediate) adds a register value and an optionally-shifted immediate value, and writes the
/// result to the destination register.
///
/// This instruction is used by the alias MOV (to/from SP).

pub struct ADD
{
    destination: Register,
    source: Register,
    immediate: u16,
    shift: bool,
    update_flags: bool,
}

impl From<u32> for ADD
{
    fn from(instruction: u32) -> Self
    {
        let sf = (instruction & 0x80000000) >> 31;
        let op = (instruction & 0x40000000) >> 30;
        let s = (instruction & 0x20000000) >> 29;
        let sh = (instruction & 0x400000) >> 22;
        let imm = (instruction & 0x3FFC00) >> 10;

        let is_64bit = sf != 0;

        // op must be unset if the operation is any ADD variant.
        debug_assert_eq!(op, 0);

        let rd = instruction & 0x1F;
        let rn = (instruction & 0x3E0) >> 5;

        ADD {
            destination: rd.into(),
            source: rn.into(),
            immediate: imm as u16,
            shift: sh != 0,
            update_flags: s != 0,
        }
    }
}

impl std::fmt::Display for ADD
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
    {
        let mneumonic = if self.update_flags { "adds" } else { "add" };

        write!(
            f,
            "{} {}, {}, #{:#X}",
            mneumonic, self.destination, self.source, self.immediate
        )?;

        if self.shift
        {
            write!(f, ", LSL #12")?;
        }

        Ok(())
    }
}
