use {
    crate::decoder::arm64::{add::ADD, adr::ADR, adrp::ADRP, stp::STP, sub::SUB},
    std::fmt::Formatter,
};

pub enum Instruction
{
    ADR(ADR),
    ADRP(ADRP),
    ADD(ADD),
    SUB(SUB),
    STP(STP),

    //
    Placeholder,
}

impl std::fmt::Display for Instruction
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
    {
        match self
        {
            Instruction::ADD(add) => write!(f, "{}", add),
            Instruction::ADR(adr) => write!(f, "{}", adr),
            Instruction::ADRP(adrp) => write!(f, "{}", adrp),
            Instruction::SUB(sub) => write!(f, "{}", sub),
            Instruction::STP(stp) => write!(f, "{}", stp),
            _ => panic!("Unhandled instruction."),
        }
    }
}
