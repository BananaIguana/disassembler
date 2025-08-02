use {crate::decoder::register::Register, instruction_gen::make_instruction};

// https://developer.arm.com/documentation/ddi0602/2022-12/Base-Instructions/ADD--immediate---Add--immediate--?lang=en

make_instruction!(
    ADD_ALT,
    check = "*00100010***********************",
    sf = 31,
    op = 30,
    S = 29,
    sh = 22,
    imm12 = 10:21,
    Rn = 5:9,
    Rd = 0:4
);
