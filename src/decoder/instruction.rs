use {
    crate::decoder::arm64::{add::ADD, adr::ADR, adrp::ADRP, movz::MOVZ, stp::STP, str::STR, stur::STUR, sub::SUB},
    std::fmt::Formatter,
};

pub enum Instruction
{
    ADR(ADR),
    ADRP(ADRP),
    ADD(ADD),
    SUB(SUB),
    STP(STP),
    MOVZ(MOVZ),
    STR(STR),
    STUR(STUR),

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
            Instruction::MOVZ(movz) => write!(f, "{}", movz),
            Instruction::STR(str) => write!(f, "{}", str),
            Instruction::STUR(stur) => write!(f, "{}", stur),
            _ => panic!("Unhandled instruction."),
        }
    }
}
