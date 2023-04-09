use {
    crate::{
        decoder::{Decoder, DecoderOutput},
        mode::Mode,
        report::{Position, Report, ReportType},
    },
    byteorder::{LittleEndian, ReadBytesExt},
    std::io::ErrorKind,
};

pub struct Arm64Decoder
{
    warnings: Vec<Report>,
    position: Position,
    mode: Mode,
}

impl Arm64Decoder {}

impl Arm64Decoder
{
    fn report_from_kind(&mut self, kind: ErrorKind) -> Result<(), Report>
    {
        let report = match kind
        {
            ErrorKind::UnexpectedEof => Report {
                description: "EOF is not 4-byte aligned.".to_string(),
                position: self.position.clone(),
                report_type: ReportType::IncompleteInstruction,
            },
            _ => panic!("Unhandled `ErrorKind`"),
        };

        if self.mode == Mode::Strict
        {
            return Err(report);
        }
        else
        {
            self.warnings.push(report);
        }

        Ok(())
    }

    fn report_from_type(&mut self, report_type: ReportType) -> Result<(), Report>
    {
        let position = self.position.clone();

        macro_rules! process_type {
            ($str:expr) => {
                Report {
                    description: $str.to_string(),
                    position,
                    report_type,
                }
            };
        }

        let report = match report_type
        {
            ReportType::IncompleteInstruction => process_type!("EOF is not 4-byte aligned."),
            ReportType::ReservedOpcode => process_type!("Encountered reserved opcode."),
            ReportType::UnallocatedOpcode => process_type!("Encountered opcode unallocated by the specification."),
            _ => process_type!("Unknown Error"),
        };

        if self.mode == Mode::Strict
        {
            return Err(report);
        }
        else
        {
            self.warnings.push(report);
        }

        Ok(())
    }
}

impl Default for Arm64Decoder
{
    fn default() -> Self
    {
        Self {
            warnings: Vec::new(),
            position: Position::default(),
            mode: Mode::Flexible,
        }
    }
}

impl Decoder for Arm64Decoder
{
    fn decoder(mode: Mode) -> Arm64Decoder
    {
        Self {
            warnings: Vec::new(),
            position: Position::default(),
            mode,
        }
    }

    fn decode(&mut self, machine_code: &[u8]) -> Result<DecoderOutput, Report>
    {
        // ARM64 instructions are always 4 bytes in size.
        let chunks = machine_code.chunks(4);

        for (idx, mut chunk) in chunks.enumerate()
        {
            self.position.byte_offset = idx * 4;
            self.position.instruction_offset = idx;

            match chunk.read_u32::<LittleEndian>()
            {
                Ok(instruction) =>
                {
                    // Initial pass to categorise opcode.
                    // https://developer.arm.com/documentation/ddi0596/2021-12/Index-by-Encoding
                    // BITS 28-25 (4 bytes)
                    let op0 = (instruction & 0x1E000000) >> 25;

                    match op0
                    {
                        // Reserved
                        0b0000 => self.report_from_type(ReportType::ReservedOpcode)?,
                        // Unallocated
                        0b0001 => self.report_from_type(ReportType::UnallocatedOpcode)?,
                        // SVE Encodings
                        0b0010 => todo!(),
                        // Unallocated
                        0b0011 => self.report_from_type(ReportType::UnallocatedOpcode)?,
                        // Data Processing (Immediate)
                        0b1000 | 0b1001 => todo!(),
                        // Branches, Exception Generating and System instructions
                        0b1010 | 0b1011 => todo!(),
                        // Load and Stores
                        0b0100 | 0b1100 | 0b0110 | 0b1110 => todo!(),
                        // Data Processing -- Register
                        0b0101 | 0b1101 => todo!(),
                        // Data Processing (Scalar FP & SIMD)
                        0b0111 | 0b1111 => todo!(),
                        _ => self.report_from_type(ReportType::UnmatchedOpcode)?,
                    }
                }
                Err(error) => self.report_from_kind(error.kind())?,
            }
        }

        Ok(DecoderOutput { _result: vec![] })
    }
}
