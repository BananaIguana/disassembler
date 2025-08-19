use {crate::decoder::register::Register, instruction_gen::make_instruction};

// https://developer.arm.com/documentation/ddi0602/2023-03/Base-Instructions/MOVZ--Move-wide-with-zero-?lang=en

make_instruction!(
    MOVZ,
    check = "?10100101???????????????????????",
    sf = 31,
    opc = 30,
    hw = 21:22,
    imm16 = 5:20,
    Rd = 0:4
);
