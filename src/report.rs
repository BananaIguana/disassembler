use std::fmt::Formatter;

#[derive(Copy, Clone)]
pub struct Position
{
    pub byte_offset: usize,
    pub instruction_offset: usize,
}

pub enum ReportType
{
    Unknown,
    ReservedOpcode,
    UnallocatedOpcode,
    UnmatchedOpcode,
    IncompleteInstruction,
}

pub struct Report
{
    pub description: String,
    pub position: Position,
    pub report_type: ReportType,
}

impl Default for Position
{
    fn default() -> Self
    {
        Self {
            byte_offset: 0,
            instruction_offset: 0,
        }
    }
}

impl std::fmt::Display for Report
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result
    {
        writeln!(f, "Description: {}", self.description)?;
        writeln!(f, "Byte Offset: {}", self.position.byte_offset)?;
        writeln!(f, "Instruction: {}", self.position.instruction_offset)
    }
}
