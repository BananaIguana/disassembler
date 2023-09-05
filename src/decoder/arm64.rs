pub mod add;
pub mod adr;
pub mod adrp;
pub mod movz;
pub mod stp;
pub mod str;
pub mod stur;
pub mod sub;

use {
    crate::{
        decoder::{
            arm64::{add::ADD, movz::MOVZ, stp::STP, str::STR, stur::STUR, sub::SUB},
            instruction::Instruction,
            Decoder,
            DecoderOutput,
        },
        mode::Mode,
        report::{Position, Report, ReportType},
    },
    bitmatch::bitmatch,
    byteorder::{LittleEndian, ReadBytesExt},
    std::io::ErrorKind,
};

pub struct Arm64Decoder
{
    instructions: Vec<Instruction>,
    warnings: Vec<Report>,
    position: Position,
    mode: Mode,
}

// https://developer.arm.com/documentation/ddi0596/2021-12/Index-by-Encoding

impl Arm64Decoder
{
    fn handle_report(&mut self, report: Report) -> Result<(), Report>
    {
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

    fn report_from_kind(&mut self, kind: ErrorKind) -> Result<(), Report>
    {
        self.handle_report(match kind
        {
            ErrorKind::UnexpectedEof => Report {
                description: "EOF is not 4-byte aligned.".to_string(),
                position: self.position,
                report_type: ReportType::IncompleteInstruction,
            },
            _ => panic!("Unhandled `ErrorKind`"),
        })
    }

    #[allow(dead_code)]
    fn report_from_type(&mut self, report_type: ReportType) -> Result<(), Report>
    {
        let position = self.position;

        macro_rules! process_type {
            ($str:expr) => {
                Report {
                    description: $str.to_string(),
                    position,
                    report_type,
                }
            };
        }

        self.handle_report(match report_type
        {
            ReportType::IncompleteInstruction => process_type!("EOF is not 4-byte aligned."),
            ReportType::ReservedOpcode => process_type!("Encountered reserved opcode."),
            ReportType::UnallocatedOpcode => process_type!("Encountered opcode unallocated by the specification."),
            ReportType::ProcessingError => process_type!("General usage error."),
            _ => process_type!("Unknown Error"),
        })
    }
}

impl Default for Arm64Decoder
{
    fn default() -> Self
    {
        Self {
            instructions: vec![],
            warnings: vec![],
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
            instructions: vec![],
            warnings: vec![],
            position: Position::default(),
            mode,
        }
    }

    #[bitmatch]
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
                    let instruction = #[bitmatch]
                    match instruction
                    {
                        "?00100010???????????????????????" => Instruction::ADD(ADD::from(instruction)),
                        "?10100010???????????????????????" => Instruction::SUB(SUB::from(instruction)),
                        "?010100010??????????????????????" => Instruction::STP(STP::from(instruction)), // Post-index
                        "?010100110??????????????????????" => Instruction::STP(STP::from(instruction)), // Pre-index
                        "?010100100??????????????????????" => Instruction::STP(STP::from(instruction)), // Signed offset
                        "?10100101???????????????????????" => Instruction::MOVZ(MOVZ::from(instruction)),
                        "1?111000000?????????01??????????" => Instruction::STR(STR::from(instruction)), // Immediate post-index
                        "1?111000000?????????11??????????" => Instruction::STR(STR::from(instruction)), // Immediate pre-index
                        "1?11100100??????????????????????" => Instruction::STR(STR::from(instruction)), // Immediate unsigned offset
                        "1?111000001?????????10??????????" => Instruction::STR(STR::from(instruction)), // Register
                        "1?111000000?????????00??????????" => Instruction::STUR(STUR::from(instruction)),
                        // "100101??????????????????????????" => println!("bl"),
                        // "1101011001011111000000?????00000" => println!("ret"),
                        // "1??10000????????????????????????" => println!("adrp"),
                        // "1?111000000?????????01??????????" => println!("str"), // immediate / post index
                        // "1?111000000?????????11??????????" => println!("str"), // immediate / pre index
                        // "1?11100100??????????????????????" => println!("str"), // immediate / unsigned offset
                        // "1?111000000?????????00??????????" => println!("stur"),
                        // "??111100?00?????????00??????????" => println!("stur"), // (SIMD&FP)
                        // "?010100011??????????????????????" => println!("ldp"),  // (post index)
                        // "?010100111??????????????????????" => println!("ldp"),  // (pre index)
                        // "?010100101??????????????????????" => println!("ldp"),  // (unsigned offset)
                        _ => Instruction::Placeholder,
                    };

                    self.instructions.push(instruction);

                    /*
                    // Initial pass to categorise opcode.
                    // https://developer.arm.com/documentation/ddi0596/2021-12/Index-by-Encoding
                    // BITS 28-25 (4 bytes)
                    let op0 = (instruction & 0x1E000000) >> 25;

                    // TIER 1
                    match op0
                    {
                        // Reserved
                        0b0000 => self.report_from_type(ReportType::ReservedOpcode)?,
                        // Unallocated
                        0b0001 => self.report_from_type(ReportType::UnallocatedOpcode)?,
                        // SVE Encodings
                        0b0010 =>
                        {} //todo!(),
                        // Unallocated
                        0b0011 => self.report_from_type(ReportType::UnallocatedOpcode)?,
                        // Data Processing (Immediate)
                        0b1000 | 0b1001 => self.dp_imm(instruction),
                        // Branches, Exception Generating and System instructions
                        0b1010 | 0b1011 =>
                        {} //todo!(),
                        // Load and Stores
                        0b0100 | 0b1100 | 0b0110 | 0b1110 => self.lns(instruction),
                        // Data Processing -- Register
                        0b0101 | 0b1101 =>
                        {} //todo!(),
                        // Data Processing (Scalar FP & SIMD)
                        0b0111 | 0b1111 =>
                        {} //todo!(),
                        _ => self.report_from_type(ReportType::UnmatchedOpcode)?,
                    }*/
                }
                Err(error) => self.report_from_kind(error.kind())?,
            }
        }

        let result = self
            .instructions
            .iter()
            .filter(|instruction| !matches!(instruction, Instruction::Placeholder))
            .map(|instruction| format!("{}", instruction))
            .collect::<Vec<String>>();

        Ok(DecoderOutput { result })
    }
}
