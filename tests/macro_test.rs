use {
    decoder::arm64::add::ADD,
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

    let decoded = ADD::from(2436763912_u32); // // add x8, x8, #0xF84
    println!("{:?}", decoded);
    println!("{}", decoded);

    let decoded = ADD::from(2440958216_u32); // // add x8, x8, #0xF84, LSL #12
    println!("{:?}", decoded);
    println!("{}", decoded);

    Ok(())
}
