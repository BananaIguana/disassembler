use {crate::decoder::register::Register, instruction_gen::make_instruction};

make_instruction!(ADD_ALT, sf = 31, op = 30, S = 29, imm12 = 10:21, Rn = 5:9, Rd = 0:4);
