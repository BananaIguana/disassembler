use {
    decoder::arm64::add_alt::ADD_ALT,
    disasm_frontend::{
        decoder::{self},
        report::Report,
    },
};

#[test]
pub fn macro_test() -> Result<(), Report>
{
    println!("Hello!");

    // let instr = 2432713725; // add x29, sp, #0x10
    // let instr = 2432730111; // add sp, sp, #0x20
    // let instr = 2432713725; // add x29, sp, #0x10
    // let instr = 2432697321; // add x9, sp, #0x0

    // let instr = 0x913E1108; // add x8, x8, #0xF84
    let instr = 0x917E1108; // add x8, x8, #0xF84 (contains LSL)
    let decoded = ADD_ALT::from(instr);

    println!("{:?}", decoded);

    Ok(())
}
